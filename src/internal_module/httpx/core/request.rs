use super::Method;
use super::ParseError;
use super::Version;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::BufRead;

/// Resource requested
#[derive(Debug, PartialEq, Clone)]
pub enum Resource {
    /// A path for a subpage
    Path(String),
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Resource::Path(s) = self;
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn parse(req: &[u8]) -> Result<Self, ParseError> {
        let mut req_header = req;
        let parsed_method;
        let parsed_version;
        let parsed_resource;
        let mut parsed_headers = HashMap::new();
        let mut body_len = 0;

        let mut total = 0;

        {
            let mut line = String::new();
            let n = req_header
                .read_line(&mut line)
                .map_err(|_| ParseError::OtherParseError)?;
            if n == 0 {
                return Err(ParseError::Pending);
            }
            let line = line.trim_end();
            total += n;

            let (method, resource, version) = process_req_line(line)?;
            parsed_method = method;
            parsed_version = version;
            parsed_resource = resource;
        }

        'exit: loop {
            let mut line = String::new();
            let n = req_header
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
                    if key.to_lowercase().as_str() == "content-length" {
                        body_len = value
                            .as_str()
                            .parse()
                            .map_err(|_| ParseError::InvalidHeaders)?;
                    }
                    parsed_headers.insert(key, value);
                } else {
                    return Err(ParseError::InvalidHeaders);
                }
            }
        }

        let body = req
            .get(total..total + body_len)
            .ok_or(ParseError::Pending)?;

        Ok(HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: body.to_vec(),
        })
    }
}

fn process_req_line(s: &str) -> Result<(Method, Resource, Version), ParseError> {
    let mut words = s.split_whitespace();
    let method = words.next().ok_or(ParseError::InvalidMethod)?;
    let resource = words.next().ok_or(ParseError::InvalidUrl)?;
    let version = words.next().ok_or(ParseError::InvalidVersion)?;

    Ok((
        method.parse()?,
        Resource::Path(resource.to_string()),
        version.parse()?,
    ))
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(':');
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.to_lowercase();
    }

    if let Some(v) = header_items.next() {
        value = v.to_string().trim_start().to_string()
    }

    (key, value)
}

impl From<HttpRequest> for String {
    #[inline(always)]
    fn from(res: HttpRequest) -> String {
        String::from(&res)
    }
}

impl From<&HttpRequest> for String {
    fn from(req: &HttpRequest) -> Self {
        let mut header_string = String::new();
        let mut length = false;
        for (k, v) in &req.headers {
            if k.to_lowercase().as_str() == "content-length" {
                length = true;
            }
            header_string.push_str(k);
            header_string.push_str(": ");
            header_string.push_str(v);
            header_string.push_str("\r\n");
        }
        if !req.body.is_empty() && !length {
            header_string.push_str(&format!("Content-Length: {}\r\n", req.body.len()));
        }

        format!(
            "{} {} {}\r\n{}\r\n",
            req.method, req.resource, req.version, header_string
        )
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_http() {
        let s: String = String::from("POST /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nContent-Length: 11\r\nAccept: */*\r\n\r\ntestbody123");
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), "localhost".into());
        headers_expected.insert("Accept".into(), "*/*".into());
        headers_expected.insert("User-Agent".into(), "curl/7.64.1".into());
        headers_expected.insert("Content-Length".into(), "11".into());
        let r = HttpRequest::parse(s.as_bytes());
        println!("{:?}", r);
        let req = r.unwrap();
        assert_eq!(Method::Post, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
        assert_eq!(req.body.as_slice(), b"testbody123");
    }

    #[test]
    fn test_display_http() {
        let req = HttpRequest {
            method: Method::Get,
            version: Version::V1_0,
            resource: Resource::Path("/abc".to_string()),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "text/html".to_string());
                h
            },
            body: b"abcd".to_vec(),
        };
        assert_eq!(
            "GET /abc HTTP/1.0\r\nContent-Type: text/html\r\nContent-Length: 4\r\n\r\n",
            String::from(&req)
        )
    }
}
