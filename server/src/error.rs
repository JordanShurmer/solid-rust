use core::fmt;

type Cause = Box<dyn  std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub cause: Option<Cause>,
}

#[derive(Debug)]
pub enum Kind {
    NotFound,
    PreconditionFailed,
    MethodNotAllowed,
    NotModified,
    NotAcceptable,
    Unexpected,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        match e.kind() {
            std::io::ErrorKind::NotFound => Error {
                kind: Kind::NotFound,
                cause: Some(e.into()),
            },

            _ => Error {
                kind: Kind::Unexpected,
                cause: Some(e.into()),
            },
        }
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Self {
        Error {
            kind: Kind::Unexpected,
            cause: Some(e.into())
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.kind)
    }
}

impl std::error::Error for Error {}
