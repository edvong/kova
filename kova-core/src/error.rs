use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Error {
    // ==========================================
    // Infrastructure Errors
    // ==========================================
    Database(Arc<dyn std::error::Error + Send + Sync>),
    Io(Arc<std::io::Error>),
    ConnectionError(String),

    // ==========================================
    // Mapping & Parsing Errors
    // ==========================================
    ColumnNotFound(String),
    TypeMismatch(String),
    ParseError(String),
    ConversionError(String),
    UnexpectedNull(String),

    // ==========================================
    // Business & Query Errors
    // ==========================================
    NotFound(String),
    InvalidParam(String),
    ValidationError(String),
    QueryError(String),
}

impl Error {
    #[inline(always)]
    pub fn database<E>(err: E) -> Self
        where
            E: std::error::Error + Send + Sync + 'static {
        Self::Database(Arc::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(e) => write!(f, "database internal error: {e}"),
            Self::Io(e) => write!(f, "file system error: {e}"),
            Self::ConnectionError(s) => write!(f, "connection error: {s}"),
            Self::ColumnNotFound(s) => write!(f, "column not found: {s}"),
            Self::TypeMismatch(s) => write!(f, "type mismatch: {s}"),
            Self::ParseError(s) => write!(f, "parse error: {s}"),
            Self::ConversionError(s) => write!(f, "conversion/serialization error: {s}"),
            Self::UnexpectedNull(s) => write!(f, "unexpected null value: {s}"),
            Self::NotFound(s) => write!(f, "record not found (ID: {s})"),
            Self::InvalidParam(s) => write!(f, "invalid parameter: {s}"),
            Self::ValidationError(s) => write!(f, "validation error: {s}"),
            Self::QueryError(s) => write!(f, "query syntax/build error: {s}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(e) => Some(&**e),
            Self::Io(e) => Some(&**e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    #[inline(always)]
    fn from(err: std::io::Error) -> Self {
        Self::Io(Arc::new(err))
    }
}
