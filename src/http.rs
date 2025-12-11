use crate::error::{Error, Result};
use std::{collections::HashMap, fmt};

pub struct Http;

/*
 * HTTP Request Structure:
 * Method Request-URI HTTP-Version CRLF
 * headers CRLF
 * message-body
 */

pub struct Request {
    method: Method,
    uri: String,
    version: Version,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Request {
    pub fn from_buffer(buffer: &[u8]) -> Result<Self> {
        let separator = b"\r\n\r\n";
        let pos = buffer
            .windows(separator.len())
            .position(|b| b == separator)
            .ok_or(Error::MissingSeparator)?;

        let head = &buffer[..pos];
        let body = buffer[pos + separator.len()..].to_vec();

        let head = String::from_utf8_lossy(head);
        let mut iter = head.split("\r\n").into_iter();

        let request_line = iter.next().ok_or(Error::MissingRequestLine)?;
        let parts: Vec<_> = request_line.split_whitespace().collect();
        let [method, request_uri, http_version] = parts.as_slice() else {
            return Err(Error::InvalidRequestLine);
        };

        let Ok(method) = Method::try_from(*method) else {
            return Err(Error::InvalidMethod);
        };

        let Ok(version) = Version::try_from(*http_version) else {
            return Err(Error::InvalidVersion);
        };

        let uri = request_uri.to_string();

        let headers: HashMap<_, _> = iter
            .map(|line| {
                if let Some((name, value)) = line.split_once(": ") {
                    let name = name.to_ascii_lowercase();
                    Ok((name.to_string(), value.to_string()))
                } else {
                    Err(Error::InvalidHeader)
                }
            })
            .collect::<Result<_>>()?;

        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }

    pub fn is_get(&self) -> bool {
        self.method == Method::Get
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }
}

/*
 * HTTP Response Structure:
 * HTTP-Version Status-Code Reason-Phrase CRLF
 * headers CRLF
 * message-body
 */

struct Response {
    method: Method,
    request_uri: String,
    version: Version,
}

enum Version {
    OnePointOne,
}

impl TryFrom<&str> for Version {
    type Error = &'static str;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let err = Err("Invalid HTTP version");

        let Some(version) = value.strip_prefix("HTTP/") else {
            return err;
        };

        let version = match version {
            "1.1" => Self::OnePointOne,
            _ => return err,
        };

        Ok(version)
    }
}

#[derive(PartialEq)]
enum Method {
    Get,
    Put,
    Post,
    Delete,
}

impl TryFrom<&str> for Method {
    type Error = &'static str;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let method = match value {
            "GET" => Self::Get,
            "PUT" => Self::Put,
            "POST" => Self::Post,
            "DELETE" => Self::Delete,
            _ => return Err("Invalid HTTP Method"),
        };

        Ok(method)
    }
}

enum StatusCode {
    Ok,
    NotFound,
    InternalServerError,
}

impl From<StatusCode> for u32 {
    fn from(value: StatusCode) -> Self {
        match value {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::InternalServerError => 505,
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ok => "OK",
            Self::NotFound => "NOT FOUND",
            Self::InternalServerError => "INTERNAL SERVER ERROR",
        };

        write!(f, "{s}")
    }
}
