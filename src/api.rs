// /**
// 为了方便使用，我将action定义为函数名，其他参数定义为函数参数
// api() 为基础函数
// */
//
// pub struct Filter(String);  //过滤器类型，文本为过滤器代码
// pub struct Rcid(u16);       //最近更改ID
// pub struct Logid(u16);      //过滤器日志ID
// pub struct JsonId(u16);     //json id


// wiki类型，每个wiki对象代表一个MediaWiki站点
// 这在一个涉及多MediaWiki控制的项目中可以十分灵活地操作不同MediaWiki站点

// impl Wiki{
//     /// 新建一个wiki类型，每个wiki对象代表一个MediaWiki站点
//     /// 这在一个涉及多MediaWiki控制的项目中可以十分灵活地操作不同MediaWiki站点
//     /// 参数 http_protocol        表示的是这个站点使用的http协议类型（http/https）
//     /// 参数 wiki_addr_root       表示的是这个站点的网址
//     /// 参数 connection_method    表示的是这个站点网址后的衔接方式，通常由站点的URL转写规则决定
//     /// ****
//     /// ** 这两个参数的意义 **
//     /// 如中文维基百科的API.php位置为：
//     /// ```https://zh.wikipedia.org/w/api.php```
//     /// 其中 ```https``` 指的是http协议
//     /// 其中 ```zh.wikipedia.org``` 指的是站点网址
//     /// 其中 ```/w/``` 指的是网址后的衔接模式
//     ///
//     pub fn new(http_protocol:HttpProtocol, wiki_addr_root:&str, connection_method:&str) -> Wiki {
//         if http_protocol == HttpProtocol::Http {
//             Wiki { http_protocol: "http://".to_owned(), wiki_addr_root: wiki_addr_root.to_owned(), connection_method: connection_method.to_owned(), proxy: build_normal_client() }
//         }else {
//             Wiki { http_protocol: "https://".to_owned(), wiki_addr_root: wiki_addr_root.to_owned(), connection_method: connection_method.to_owned(), proxy: build_normal_client() }
//         }
//     }
//
//     fn get_url_prefix(&self)->String{
//         self.http_protocol.to_owned() + &self.wiki_addr_root + &self.connection_method
//     }
//
//     fn set_proxy(&mut self, proxy_url:&str){
//         let client = build_client(proxy_url);
//         self.proxy = client;
//     }
// }


// /// 检查以查看防滥用过滤器是否匹配某个变量集、某次编辑或某条日志记载的过滤器活动
// /// 需要vars、rcid或logid，然而只有其中一个会用到。
// /// **filter**: 要检查是否匹配的完整过滤器文本。 **(必需)**
// /// **vars**: 要测试是否触发的JSON编码数组变量。
// /// **rcid**: 要检查是否触发的最近更改ID。
// /// **logid**: 用以检查的滥用过滤器日志ID。
//
// pub trait AllowToCheckFilterMatch{ fn get_url_para(&self)->String;}
// impl AllowToCheckFilterMatch for JsonId{
//     fn get_url_para(&self)->String{
//         "&vars=".to_owned() + &self.0.to_string()
//     }
// }
// impl AllowToCheckFilterMatch for Rcid{
//     fn get_url_para(&self)->String{
//         "&rcid=".to_owned() + &self.0.to_string()
//     }
// }
// impl AllowToCheckFilterMatch for Logid{
//     fn get_url_para(&self)->String{
//         "&logid=".to_owned() + &self.0.to_string()
//     }
// }
// impl Wiki {
//     // 这里的返回值传递需要下功夫研究一番
//     pub fn api(&self, parameter:String) -> String{
//         // get(self.get_url_prefix() + "api.php?action=" + &parameter, &self.proxy)
//         "".to_owned()
//     }
//
//     pub async fn abuse_filter_check_match<T: AllowToCheckFilterMatch>(&self, filter: String, target: T) ->Result<String, Box<dyn Error>> {
//         let result = Self::api(&self, "abusefiltercheckmatch".to_owned() + &filter + &target.get_url_para());
//         Ok(result)
//     }
//
//
// }
