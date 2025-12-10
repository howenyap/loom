use std::{fmt, path::Display};

pub struct Http;

/*
 * HTTP Request Structure:
 * Method Request-URI HTTP-Version CRLF
 * headers CRLF
 * message-body
 */

/*
 * HTTP Response Structure:
 * HTTP-Version Status-Code Reason-Phrase CRLF
 * headers CRLF
 * message-body
 */

enum StatusCode {
    OK,
}

impl Into<u32> for StatusCode {
    fn into(self) -> u32 {
        match self {
            Self::OK => 200,
        }
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OK => "OK",
        };

        write!(f, "{s}")
    }
}
