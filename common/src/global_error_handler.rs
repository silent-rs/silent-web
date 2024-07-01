use crate::response::Res;
use silent::{Configs, Response, SilentError};

// TODO: 待去除
#[allow(dead_code)]
pub async fn exception_handler(err: SilentError, _configs: Configs) -> Response {
    Res::<()>::with_err(&err.message()).into()
}
