//! a
/// a
#[derive(Debug)]
pub struct Error {
    inner: Box<ErrorImpl>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Error {}
#[derive(Debug)]
struct ErrorImpl {
    kind: Kind,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}
#[derive(Debug)]
pub(super) enum Kind {
    Popsicle(String),
}
impl Error {
    fn new(kind: Kind) -> Error {
        Error { inner: Box::new(ErrorImpl { kind, cause: None }) }
    }
    fn with<C: Into<Box<dyn std::error::Error + Send + Sync>>>(mut self, cause: C) -> Error {
        self.inner.cause = Some(cause.into());
        self
    }
    ///
    pub fn new_popsicle(kind: &str, message: &str) -> Error {
        Error::new(Kind::Popsicle(kind.to_owned()))
            .with(Error::new(Kind::Popsicle(message.to_owned())))
    }
}
