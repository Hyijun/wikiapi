pub mod network;
pub mod api;
mod wiki;
mod page;
mod user;
mod group;
mod wiki_depot;
mod json;
mod file;


#[cfg(test)]
mod tests {
    use crate::network::{get_text, WikiClient};
    use crate::api;
    use std::fmt::Display;
    use crate::wiki::Wiki;
    use std::collections::HashMap;
    use crate::group::Group;
    use serde_json::{Value, Map};
    use std::ptr::null;


    //测试network模块的可用性
    #[test]
    fn network_test() {
        let c = WikiClient::new()
            .set_proxy("http://127.0.0.1:10809");
        let r = get_text("https://zh.wikipedia.org/".to_owned(), &c).unwrap_or_else(|e| e.to_string());
        // println!("{}", r);
        let g: HashMap<String, Group> = HashMap::new();
        let w = Wiki::new("https://", "zh.wikipedia.org", "/w/api.php", g, c);


        // let mut r = w.api("action=query&format=json&list=allusers&aufrom=Hyijun".to_owned()).unwrap();
        // println!("{:?}", r["query"]["allusers"].as_array().unwrap().get(0).unwrap().as_object().unwrap().get("name").unwrap());

        // let r = w.page_id("Wikipedia:首页");
        println!("{:}", r.unwrap());

        // let r

    }
}

