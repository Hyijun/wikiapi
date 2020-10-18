pub mod network;
pub mod api;


#[cfg(test)]
mod tests {
    use crate::network::{get_text, build_client};
    use crate::api;

    //测试network模块的可用性
    #[test]
    fn network_test(){
        let c = build_client("http://127.0.0.1:1085");
        let s : String = get_text("https://zh.wikipedia.org".to_string(), &c);
        assert_eq!(s, "");
    }
}

