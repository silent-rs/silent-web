use silent::{SilentError, StatusCode};
use thiserror::Error;

/// SilentError is the error type for the `silent` library.
#[derive(Error, Debug)]
pub enum WebError {
    /// 业务错误
    #[error("业务错误: {msg} ({code})")]
    BusinessError {
        /// 错误码
        code: StatusCode,
        /// 错误信息
        msg: String,
    },
}

impl From<WebError> for SilentError {
    fn from(value: WebError) -> Self {
        match value {
            WebError::BusinessError { code, msg } => Self::business_error(code, msg),
        }
    }
}
