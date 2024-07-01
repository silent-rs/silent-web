use serde::Serialize;

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
pub struct Res<T> {
    pub code: Option<i32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}

/// 填入到extensions中的数据
#[derive(Debug, Clone)]
// TODO: 待去除
#[allow(dead_code)]
pub struct ResJsonString(pub String);

impl<T: Serialize> Res<T> {
    // TODO: 待去除
    #[allow(dead_code)]
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
    // TODO: 待去除
    #[allow(dead_code)]
    pub fn with_msg(msg: &str) -> Self {
        Self {
            code: Some(200),
            data: None,
            msg: Some(msg.to_string()),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some(msg.to_string()),
        }
    }
}
