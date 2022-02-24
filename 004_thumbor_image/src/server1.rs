use axum::{extract::Path, routing::get, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;

mod pb;

use pb::*;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new()
        // `GET /image` 会执行 generate 函数，并把 spec 和 url 传递过去
        .route("/image/:spec/:url", get(generate));

    // 运行 web 服务器
    let addr = "127.0.0.1:3000".parse().unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    println!("received generate {:#?}, {:#?}", spec, url);
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}

#[cfg(test)]
mod tests {
    use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

    #[test]
    fn get_encode_url_should_be_working() {

       let url = "https://www.rust-lang.org/";
        println!("rust-lang url: {}", utf8_percent_encode(url, NON_ALPHANUMERIC).to_string());
        assert_eq!(utf8_percent_encode(url, NON_ALPHANUMERIC).to_string(), "https%3A%2F%2Fwww%2Erust%2Dlang%2Eorg%2F");
    }
}
