use std::fmt;

pub struct Http;

/*
 * HTTP Request Structure:
 * Method Request-URI HTTP-Version CRLF
 * headers CRLF
 * message-body
 */

struct Request {
    method: Method,
    request_uri: String,
    version: Version,
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

enum Method {
    Get,
}

enum StatusCode {
    Ok,
}

impl From<StatusCode> for u32 {
    fn from(value: StatusCode) -> Self {
        match value {
            StatusCode::Ok => 200,
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ok => "OK",
        };

        write!(f, "{s}")
    }
}
