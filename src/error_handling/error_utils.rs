use crate::error_handling::error::Error;
use crate::error_handling::result::Result;

pub trait ErrorExt<T> {
    fn wrap(self, message: &str) -> Result<T>;
    fn wrap_with(self, error: Error) -> Result<T>;
}

impl<T, E: Into<Error>> ErrorExt<T> for std::result::Result<T, E> {
    fn wrap(self, message: &str) -> Result<T> {
        self.map_err(|e| Error::UnexpectedError(format!("{}: {}", message, e.into())))
    }

    fn wrap_with(self, error: Error) -> Result<T> {
        self.map_err(|_| error)
    }
}

#[macro_export]
macro_rules! try_with_context {
    ($expression:expr, $context:expr) => {
        $expression.map_err(|e| {
            let error_message = format!("{}: {}", $context, e);
            crate::error_handling::error::Error::UnexpectedError(error_message)
        })
    };
}

pub fn wrap_error<T, E: Into<Error>>(result: std::result::Result<T, E>, message: &str) -> Result<T> {
    result.map_err(|e| Error::UnexpectedError(format!("{}: {}", message, e.into())))
}