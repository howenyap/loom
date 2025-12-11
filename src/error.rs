pub enum Error {
    InvalidHeader,
    InvalidRequestLine,
    InvalidMethod,
    InvalidVersion,
    MissingRequestLine,
    MissingSeparator,
}

pub type Result<T> = std::result::Result<T, Error>;
