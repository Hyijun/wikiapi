extern crate reqwest;
extern crate tokio;

use reqwest::Client;
use reqwest::Proxy;
use std::error::Error;
use tokio::runtime::Runtime;

pub fn get_text_directly(url: String) -> String{
    async fn get_(url_:String) -> Result<String, Box<dyn Error>>{
        let resp = reqwest::get(&url_)
            .await?
            .text()
            .await?;
        Ok(resp)
    }
    Runtime::new()
        .expect("在建立tokio运行时时出错")
        .block_on(get_(url)).unwrap()
}

pub fn get_text(url:String, client:&Client) -> String{
    async fn get_(url_:String, c:&Client) -> Result<String, Box<dyn Error>>{
        let resp  = c.get(&url_)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
    Runtime::new()
        .expect("在建立tokio运行时时出错")
        .block_on(get_(url, client)).unwrap()
}

///
/// **异步**方法，http的get()方法，需要一个协议为http/https的URL字符串以及一个client类型的请求构造器引用
/// ***
/// example
/// ```
/// use wikiapi::network::{build_client, get};
/// let c = build_client("http://127.0.0.1:1080");
/// let result = get("https://blog.hyijun.com".to_string(), &c);
/// ```
///
pub async fn get(url :String, client:&Client) -> Result<String, Box<dyn Error>>{
    let resp = client.get(&url)
        .send()
        .await?
        .text()
        .await?;
    Ok(resp)
}

pub fn build_client(proxy_url:&str) -> Client{
    async fn create(pu:&str) -> Result<Client, Box<dyn Error>>{
        let c = Client::builder()
            .proxy(Proxy::all(pu)?)
            .build()?;
        Ok(c)
    }
    Runtime::new()
        .expect("在建立tokio运行时时出错")
        .block_on(create(proxy_url)).unwrap()
}

pub fn build_normal_client() -> Client{
    async fn create() -> Result<Client, Box<dyn Error>>{
        let c = Client::builder()
            .build()?;
        Ok(c)
    }
    Runtime::new()
        .expect("在建立tokio运行时时出错")
        .block_on(create()).unwrap()
}
