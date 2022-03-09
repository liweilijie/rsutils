use chrono::{Local, Utc};
use serde::{Deserialize, Serialize};
use url::form_urlencoded::byte_serialize;
use crate::{Client, Error};
use crate::Result;
use crypto::mac::Mac;
use crypto::hmac::Hmac;
use crypto::sha1::Sha1;
use tracing::info;

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

impl Client {
    pub async fn send_sms<T: Serialize>(&self, req: SmsRequest<T>) -> Result<()> {
        let nonce = format!("{}", Local::now().timestamp_subsec_nanos());
        let ts = format!("{}", Utc::now().format("%Y-%m-%dT%H:%M:%SZ"));
        let param = serde_json::to_string(&req.param).unwrap();
        let phones = req.phones.join(",");

        let mut params = Vec::from(FIXED_SMS_PARAM);
        params.push(("AccessKeyId", &self.access_key));
        params.push(("SignName", &req.sign_name));
        params.push(("TemplateCode", &req.template_code));
        params.push(("Timestamp", &ts));
        params.push(("TemplateParam", &param));
        params.push(("SignatureNonce", &nonce));
        params.push(("PhoneNumbers", &phones));
        params.sort_by_key(|item| item.0);
        let params: Vec<String> = params
            .into_iter()
            .map(|(k,v)|format!("{}={}", special_url_encode(k), special_url_encode(v)))
            .collect();
        let sorted_query_string = params.join("&");
        info!("sorted_query_string: {}", sorted_query_string);
        let string_to_sign = format!(
            "GET&{}&{}",
            special_url_encode("/"),
            special_url_encode(&sorted_query_string)
        );
        info!("string_to_sign: {}", string_to_sign);
        let sign = sign(format!("{}&", self.secret_key), &string_to_sign);
        info!("sign: {}", sign);
        let signature = special_url_encode(&sign);
        let final_url = format!(
            "https://dysmsapi.aliyuncs.com/?Signature={}&{}",
            signature, sorted_query_string
        );
        info!("final_url: {}", final_url);
        self.http
            .get(&final_url)
            .send().await?
            .json::<SmsResponse>()
            .await.
            map_err(From::from)
            .and_then(|resp| {
                if resp.code.eq("OK") {
                    info!("send sms success: {resp:#?}");
                    Ok(())
                } else {
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

fn sign<S: Into<String>>(key: S, body: &str) -> String {
    let mut mac = Hmac::new(Sha1::new(), key.into().as_bytes());
    mac.input(body.as_bytes());
    let result = mac.result();
    let code = result.code();
    base64::encode(code)
}