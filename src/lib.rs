pub mod network;
pub mod api;
mod wiki;
mod page;
mod user;
mod group;


#[cfg(test)]
mod tests {
    use crate::network::{get_text, build_client};
    use crate::api;
    use std::fmt::Display;

    //测试network模块的可用性
    #[test]
    fn network_test(){
        let c = build_client("http://127.0.0.1:1085");
        let string = match get_text("https://zh.wikipedia.org".to_string(), &c) {
            Ok(s) => s,
            Err(e) => e.to_string()
        };
        println!("{}", string);
    }
}

