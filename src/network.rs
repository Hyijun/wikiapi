extern crate reqwest;
extern crate tokio;

use reqwest::Client;
use reqwest::Proxy;
use std::error::Error;
use tokio::runtime::Runtime;
use std::fmt::Formatter;

#[derive(Clone)]
pub struct WikiClient {
    client: Client
}

impl WikiClient {
    pub fn new() -> WikiClient {
        async fn create() -> Result<Client, Box<dyn Error>> {
            let c = Client::builder()
                .build()?;
            Ok(c)
        }
        let client = Runtime::new()
            .expect("在建立tokio运行时时出错")
            .block_on(create()).unwrap();
        WikiClient { client }
    }

    pub fn set_proxy(mut self, proxy_url: &str) -> Self {
        async fn create(pu: &str) -> Result<Client, Box<dyn Error>> {
            let c = Client::builder()
                .proxy(Proxy::all(pu)?)
                .build()?;
            Ok(c)
        }
        let c = Runtime::new()
            .expect("在建立tokio运行时时出错")
            .block_on(create(&*proxy_url)).unwrap();
        self.client = c;
        self
    }
}

pub fn get_text(url: String, client: &WikiClient) -> Result<String, Box<NetworkError>> {
    async fn get_(url_: String, c: &WikiClient) -> Result<String, Box<dyn Error>> {
        let resp = c.client.get(&url_)
            .send()
            .await?
            .text()
            .await?;
        Ok(resp)
    }
    Runtime::new()
        .expect("在建立tokio运行时时出错")
        .block_on(get_(url, client)).map_err(|e| -> Box<NetworkError> { Box::new(NetworkError::new(e.to_string())) })
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct NetworkError { info: String }

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl NetworkError {
    pub fn new(info: String) -> NetworkError {
        NetworkError { info }
    }
}

impl std::error::Error for NetworkError {}

//
// pub fn get_text_directly(url: String) -> String{
//     async fn get_(url_:String) -> Result<String, Box<dyn Error>>{
//         let resp = reqwest::get(&url_)
//             .await?
//             .text()
//             .await?;
//         Ok(resp)
//     }
//     Runtime::new()
//         .expect("在建立tokio运行时时出错")
//         .block_on(get_(url)).unwrap()
// }


// ///
// /// **异步**方法，http的get()方法，需要一个协议为http/https的URL字符串以及一个client类型的请求构造器引用
// /// ***
// /// example
// /// ```
// /// use wikiapi::network::{build_client, get};
// /// let c = build_client("http://127.0.0.1:1080");
// /// let result = get("https://blog.hyijun.com".to_string(), &c);
// /// ```
// ///
// pub async fn get(url :String, client:&Client) -> Result<String, Box<dyn Error>>{
//     let resp = client.get(&url)
//         .send()
//         .await?
//         .text()
//         .await?;
//     Ok(resp)
// }


