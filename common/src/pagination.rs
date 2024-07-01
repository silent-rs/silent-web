use serde::Serialize;

/// 数据统一返回格式
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Pagination<T> {
    data: Vec<T>,
    page_size: u64,
    page_no: u64,
    total: u64,
}
