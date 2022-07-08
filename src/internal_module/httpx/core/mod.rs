// from https://github.com/Squioole/http
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod chunk;
pub mod request;
pub mod response;

/// Http method
#[derive(Debug, PartialEq, Clone)]
pub enum Method {
    Get,
    Post,
    Head,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self).to_uppercase())
    }
}

impl FromStr for Method {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::Get),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "CONNECT" => Ok(Method::Connect),
            "HEAD" => Ok(Method::Head),
            "OPTIONS" => Ok(Method::Options),
            "TRACE" => Ok(Method::Trace),
            "PATCH" => Ok(Method::Patch),
            _ => Err(ParseError::InvalidMethod),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    V1_0,
    V1_1,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Version::V1_0 => f.write_str("HTTP/1.0"),
            Version::V1_1 => f.write_str("HTTP/1.1"),
        }
    }
}

impl FromStr for Version {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.0" => Ok(Version::V1_0),
            "HTTP/1.1" => Ok(Version::V1_1),
            _ => Err(ParseError::InvalidVersion),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    /// Any parse error not part of this list.
    OtherParseError,
    /// Invalid HTTP method.
    InvalidMethod,
    /// Invalid URL.
    InvalidUrl,
    /// Invalid HTTP version.
    InvalidVersion,
    /// Invalid request line.
    InvalidRequestLine,
    /// Invalid status code.
    InvalidStatusCode,
    /// Invalid status line.
    InvalidStatusLine,
    /// Invalid header section.
    InvalidHeaders,
    /// Invalid chunk data.
    InvalidChunk,
    /// Pending
    Pending,
}
