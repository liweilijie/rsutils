use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::byte_serialize;
use crate::{Client, Error};
use crate::Result;
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use tracing::{error, info, debug};

pub struct SmsRequest<T: Serialize> {
    pub phones: Vec<String>,
    pub sign_name: String,
    pub template_code: String,
    pub out_id: Option<String>,
    pub param: T,
}

const FIXED_SMS_PARAM: &[(&str, &str)] = &[
    ("Action", "SendSms"),
    ("Format", "JSON"),
    ("RegionId", "cn-hangzhou"),
    ("SignatureMethod", "HMAC-SHA1"),
    ("SignatureVersion", "1.0"),
    ("Version", "2017-05-25"),
];

// 按这里面的原型实现的：https://help.aliyun.com/document_detail/101343.html
impl Client {
    pub async fn send_sms<T: Serialize>(&self, req: SmsRequest<T>) -> Result<()> {
        let nonce = format!("{}", Local::now().timestamp_subsec_nanos());
        let ts = format!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%SZ"));
        let param = serde_json::to_string(&req.param).unwrap();
        let phones = req.phones.join(",");

        // 1. 指定请求参数，并且请求参数中不允许出现以Signature为key的参数。
        let mut params = Vec::from(FIXED_SMS_PARAM);
        params.push(("AccessKeyId", &self.access_key));
        params.push(("SignName", &req.sign_name));
        params.push(("TemplateCode", &req.template_code));
        params.push(("Timestamp", &ts));
        params.push(("TemplateParam", &param));
        params.push(("SignatureNonce", &nonce));
        params.push(("PhoneNumbers", &phones));
        // 2. 根据参数Key排序（顺序）
        params.sort_by_key(|item| item.0);
        // 3. 构造待签名的请求串
        // 首先介绍下面会用到的特殊URL编码这个是POP特殊的一种规则，即在一般的URLEncode后再增加三种字符替换：
        // 加号（+）替换成 %20、星号（*）替换成 %2A、%7E 替换回波浪号（~）

        // 构造待签名的请求串：
        // 3.1 把排序后的参数顺序拼接成如下格式：
        // * specialUrlEncode(参数Key) + "=" + specialUrlEncode(参数值)
        let params: Vec<String> = params
            .into_iter()
            .map(|(k,v)|format!("{}={}", special_url_encode(k), special_url_encode(v)))
            .collect();
        let sorted_query_string = params.join("&");
        debug!("sorted_query_string: {}", sorted_query_string);
        // 3.2 按POP的签名规则拼接成最终的待签名串。
        // 规则如下：
        // * HTTPMethod + “&” + specialUrlEncode(“/”) + ”&” + specialUrlEncode(sortedQueryString)
        let string_to_sign = format!(
            "GET&{}&{}",
            special_url_encode("/"),
            special_url_encode(&sorted_query_string)
        );
        // 这就完成了待签名的请求字符串。打印结果如下：
        debug!("string_to_sign: {}", string_to_sign);
        // 特别说明：POP要求需要后面多加一个“&”字符，即accessSecret + “&”
        let sign = sign(format!("{}&", self.secret_key), &string_to_sign);
        debug!("sign: {}", sign);
        // 4. 增加签名结果到请求参数中，发送请求
        // 签名也要做特殊URL编码。
        let signature = special_url_encode(&sign);
        // 最终合法的 url
        let final_url = format!(
            "https://dysmsapi.aliyuncs.com/?Signature={}&{}",
            signature, sorted_query_string
        );
        debug!("final_url: {}", final_url);
        self.http
            .get(&final_url)
            .send().await?
            .json::<SmsResponse>()
            .await.
            map_err(From::from)
            .and_then(|resp| {
                if resp.code.eq("OK") {
                    info!("send sms success: {:#?}", resp);
                    Ok(())
                } else {
                    error!("send sms failed: {:#?}", resp);
                    Err(Error::Internal(resp.message))
                }
            })
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct SmsResponse {
    #[serde(rename = "Message")]
    message: String,
    #[serde(rename = "RequestId")]
    request_id: String,
    #[serde(rename = "BizId")]
    biz_id: String,
    #[serde(rename = "Code")]
    code: String,
}

fn special_url_encode(s: &str) -> String {
    let s: String = byte_serialize(s.as_bytes()).collect();
    s.replace("+", "%20")
        .replace("*", "%2A")
        .replace("%7E", "~")
}

// 对于每一次HTTP或HTTPS协议请求，我们会根据访问中的签名信息验证访问请求者身份。
// 具体由使用AccessKeyID和AccessKeySecret对称加密验证实现。其中AccessKeyID是访问者身份，
// AccessKeySecret是加密签名字符串和服务器端验证签名字符串的密钥，必须严格保密谨防泄露。
// 签名采用HmacSHA1算法 + Base64，编码采用UTF-8。参考代码如下：
// 参数说明：
// key: 您的AccessKeyId对应的密钥AccessSecret，特别说明：POP要求需要后面多加一个“&”字符，即accessSecret + “&”
// body: 即第三步生成的待签名请求串
fn sign<S: Into<String>>(key: S, body: &str) -> String {
    let mut mac = Hmac::new(Sha1::new(), key.into().as_bytes());
    mac.input(body.as_bytes());
    let result = mac.result();
    let code = result.code();
    base64::encode(code)
}