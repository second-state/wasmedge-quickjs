use super::ParseError;
use super::Version;
use std::collections::HashMap;
use std::fmt::Display;
use std::io::BufRead;

#[derive(Debug, PartialEq, Clone)]
pub enum BodyLen {
    Length(usize),
    Chunked,
}

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse {
    pub version: Version,
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body_len: BodyLen,
}

impl Default for HttpResponse {
    fn default() -> Self {
        Self {
            version: Version::V1_0,
            status_code: 200,
            status_text: "OK".to_string(),
            headers: HashMap::default(),
            body_len: BodyLen::Length(0),
        }
    }
}

impl HttpResponse {
    #[deprecated]
    pub fn new(status_code: u16, headers: Option<HashMap<String, String>>) -> HttpResponse {
        let mut response = HttpResponse::default();
        response.status_code = status_code;
        if let Some(headers) = headers {
            response.headers = headers;
        }
        response.status_text = match response.status_code {
            200 => "OK",
            400 => "Bad Request",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "",
        }
        .to_string();

        response
    }
}

impl From<HttpResponse> for String {
    #[inline(always)]
    fn from(res: HttpResponse) -> String {
        String::from(&res)
    }
}

impl From<&HttpResponse> for String {
    fn from(res: &HttpResponse) -> String {
        let mut header_string = String::new();
        for (k, v) in &res.headers {
            header_string.push_str(k);
            header_string.push_str(": ");
            header_string.push_str(v);
            header_string.push_str("\r\n");
        }

        match res.body_len {
            BodyLen::Length(0) => {}
            BodyLen::Length(len) => {
                header_string.push_str(&format!("Content-Length: {}\r\n", len));
            }
            BodyLen::Chunked => {
                header_string.push_str(&format!("Transfer-Encoding: chunked\r\n"));
            }
        }

        format!(
            "{} {} {}\r\n{}\r\n",
            res.version, res.status_code, res.status_text, header_string,
        )
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl HttpResponse {
    pub fn from_status(
        headers: Option<HashMap<String, String>>,
        status_code: u16,
        status_text: &str,
    ) -> Self {
        let mut response = HttpResponse::default();

        response.status_code = status_code;
        if let Some(headers) = headers {
            response.headers = headers;
        }

        response.status_text = status_text.to_string();
        response
    }

    pub fn parse(mut req: &[u8]) -> Result<(Self, usize), ParseError> {
        let parsed_version;
        let parsed_status;
        let parsed_status_text;
        let mut parsed_headers = HashMap::new();
        let mut parsed_body_len = BodyLen::Length(0);

        let mut total = 0;

        {
            let mut line = String::new();
            let n = req
                .read_line(&mut line)
                .map_err(|_| ParseError::OtherParseError)?;
            if n == 0 {
                return Err(ParseError::Pending);
            }
            let line = line.trim_end();
            total += n;

            let (version, status, status_text) = process_resp_line(line)?;
            parsed_version = version;
            parsed_status = status;
            parsed_status_text = status_text;
        }

        'exit: loop {
            let mut line = String::new();
            let n = req
                .read_line(&mut line)
                .map_err(|_| ParseError::OtherParseError)?;
            if n == 0 {
                return Err(ParseError::Pending);
            }
            let line = line.trim_end();
            total += n;

            {
                if line.is_empty() {
                    break 'exit;
                } else if line.contains(':') {
                    let (key, value) = process_header_line(line);
                    let header_key = key.to_lowercase();
                    if header_key.as_str() == "content-length" {
                        let len = value.parse().map_err(|_| ParseError::InvalidHeaders)?;
                        parsed_body_len = BodyLen::Length(len);
                    } else if header_key.as_str() == "transfer-encoding"
                        && value.as_str() == "chunked"
                    {
                        parsed_body_len = BodyLen::Chunked;
                    }
                    parsed_headers.insert(key, value);
                } else {
                    return Err(ParseError::InvalidHeaders);
                }
            }
        }

        if let BodyLen::Length(n) = parsed_body_len {
            if req.len() < n {
                return Err(ParseError::Pending);
            }
        }

        Ok((
            HttpResponse {
                version: parsed_version,
                status_code: parsed_status,
                status_text: parsed_status_text,
                headers: parsed_headers,
                body_len: parsed_body_len,
            },
            total,
        ))
    }
}

fn process_resp_line(s: &str) -> Result<(Version, u16, String), ParseError> {
    let mut words = s.split_whitespace();
    let version = words.next().ok_or(ParseError::InvalidStatusLine)?;
    let status = words.next().ok_or(ParseError::InvalidStatusLine)?;
    let resource = words.next().unwrap_or_default();

    Ok((
        version.parse()?,
        status.parse().map_err(|_| ParseError::InvalidStatusCode)?,
        resource.to_string(),
    ))
}
fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string().trim_start().to_string()
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_expected = HttpResponse {
            version: Version::V1_0,
            status_code: 200,
            status_text: "OK".to_string(),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                h
            },
            body_len: BodyLen::Length(10),
        };

        assert_eq!(
            "HTTP/1.0 200 OK\r\nContent-Type: text/html\r\nContent-Length: 10\r\n\r\n",
            format!("{}", response_expected)
        );
    }

    #[test]
    fn test_response_struct_parse() {
        let data = "HTTP/1.0 200 OK\r\nContent-Length: 4\r\nContent-Type: text/html\r\n\r\nhaha";
        let (resp, n) = HttpResponse::parse(data.as_bytes()).unwrap();
        assert_eq!(Version::V1_0, resp.version);
        assert_eq!(200, resp.status_code);
        assert_eq!("OK", resp.status_text);
        assert_eq!(BodyLen::Length(4), resp.body_len);
        assert_eq!("haha", data.split_at(n).1)
    }
}
