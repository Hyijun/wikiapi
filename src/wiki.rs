extern crate reqwest;
extern crate tokio;

use crate::page::Page;
use crate::user::User;
use crate::group::Group;
use crate::wiki_depot::{PagesDepot, UsersDepot};
use crate::network::{WikiClient, get_text, NetworkError};
use std::fmt::{Formatter, Display};
use std::collections::HashMap;
use crate::json::{get_json_string, JsonError};
use serde::de::Error;
use std::fmt;
use serde_json::Value;

#[derive(Clone)]
pub struct Wiki {
    pages_depot: Vec<PagesDepot>,
    users_depot: Vec<UsersDepot>,
    groups: HashMap<String, Group>,

    client: WikiClient,
    protocol: HttpProtocol,
    address: String,
    api_path: String,
}

impl Wiki {
    pub fn new<T: Display>(protocol: T, addr: T, api_path: T, groups: HashMap<String, Group>, client: WikiClient) -> Wiki {
        let s = &protocol.to_string().to_lowercase();
        let p = match &s[..] {
            "http" => HttpProtocol::Http,
            "https" => HttpProtocol::Https,
            "https://" => HttpProtocol::Https,
            "http://" => HttpProtocol::Http,
            _ => HttpProtocol::Http
        };
        let mut w = Wiki {
            pages_depot: Vec::new(),
            users_depot: Vec::new(),
            groups,
            client,
            protocol: p,
            address: addr.to_string(),
            api_path: api_path.to_string(),
        };
        w.pages_depot.push(PagesDepot::new());
        w.users_depot.push(UsersDepot::new());
        w
    }
    pub fn from_old<T: Display>(
        protocol: T,
        addr: T,
        api_path: T,
        groups: HashMap<String, Group>,
        client: WikiClient,
        pages_depot: Vec<PagesDepot>,
        users_depot: Vec<UsersDepot>,
    ) -> Wiki {
        let s = &protocol.to_string().to_lowercase();
        let p = match &s[..] {
            "http" => HttpProtocol::Http,
            "https" => HttpProtocol::Https,
            "https://" => HttpProtocol::Https,
            "http://" => HttpProtocol::Http,
            _ => HttpProtocol::Http
        };
        Wiki {
            pages_depot,
            users_depot,
            groups,
            client,
            protocol: p,
            address: addr.to_string(),
            api_path: api_path.to_string(),
        }
    }

    pub fn add_pages_depot(&mut self) -> u32 {
        self.pages_depot.push(PagesDepot::new());
        self.get_pages_depot_len()
    }

    pub fn add_users_depot(&mut self) -> u32 {
        self.users_depot.push(UsersDepot::new());
        self.get_users_depot_len()
    }

    pub fn get_pages_depot_len(&self) -> u32 {
        self.pages_depot.len() as u32
    }

    pub fn get_users_depot_len(&self) -> u32 {
        self.users_depot.len() as u32
    }

    pub fn get_pages_amount(&self) -> u32 {
        let mut i: u32 = 0;
        for each in &self.pages_depot {
            i += each.get_amount() as u32;
        }
        i
    }

    pub fn get_users_amount(&self) -> u32 {
        let mut i: u32 = 0;
        for each in &self.users_depot {
            i += each.get_amount() as u32;
        }
        i
    }
}


#[derive(Clone, Eq, PartialEq)]
pub enum HttpProtocol {
    Http,
    Https,
}

impl std::fmt::Display for HttpProtocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpProtocol::Http => write!(f, "http://"),
            HttpProtocol::Https => write!(f, "https://")
        }
    }
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ApiRequestError {
    JsonError(JsonError),
    NetworkError(NetworkError),
}

impl std::error::Error for ApiRequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            ApiRequestError::JsonError(ref e) => Some(e),
            ApiRequestError::NetworkError(ref e) => Some(e)
        }
    }
}


impl std::fmt::Display for ApiRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            ApiRequestError::JsonError(ref e) => e.fmt(f),
            ApiRequestError::NetworkError(ref e) => e.fmt(f)
        }
    }
}

impl Wiki {
    pub fn api(&self, param: String) -> Result<Value, Box<ApiRequestError>> {
        let u = self.protocol.to_string() + &self.address + &self.api_path + "?" + &param;
        let s = get_text(u, &self.client);
        let s1 = s.map_err(|e| -> Box<ApiRequestError>{ Box::new(ApiRequestError::NetworkError(NetworkError::new(e.to_string()))) })?;
        get_json_string(&s1).map_err(|e| -> Box<ApiRequestError>{ Box::new(ApiRequestError::JsonError(JsonError::new(e.to_string()))) })
    }

    pub fn page_info(&self, name: String){

    }

//     pub fn page_id(&self, name:&str) -> Result<u32, Box<ApiRequestError>> {
//         self.api("action=query&format=json&formatversion=2&prop=contributors&titles=".to_owned() + &name)?
//             ["query"]["pages"]
//             .as_array()
//             .ok_or_else(|| -> Box<ApiRequestError> {Box::new(ApiRequestError::JsonError(JsonError::new("cannot slove json when try to get page id.".to_string())))})?
//             [0]["pageid"].clone().as_u64().map(|i| -> u32{i as u32}).unwrap_or_else(|| -> Box<ApiRequestError>{Box::new(ApiRequestError::JsonError(JsonError::new("cannot slove json when try to get page id.".to_owned()))})
//     }
}