use serde::Serialize;

// 业务板块：${name}，发生异常原因：${code}，请及时处理。
#[derive(Debug, Serialize)]
pub struct SmsParam {
    pub name: String,
    pub code: String,
}