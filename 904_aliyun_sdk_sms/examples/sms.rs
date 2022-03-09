use anyhow::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Sending SMS");

    let ak = "LTAI5t6SBdCNdURqbD4jumaM";
    let sk = "MSevUswTfVxwKaayJad5iGAe9lKfzJ";

    let template_code = "SMS_235793799";
    let sign_name = "恒乐淘";


    let p = aliyun_sdk::SmsParam {
        name: "rust test".to_string(),
        code: "20220309".to_string()
    };

    let sms = aliyun_sdk::SmsRequest {
        phones: vec!["18180815129".to_string()],
        sign_name: sign_name.to_string(),
        template_code: template_code.to_string(),
        out_id: Some("123".to_string()),
        param: p,
    };

    let client = aliyun_sdk::Client::new(ak, sk);
    client.send_sms(sms).await?;

    Ok(())
}