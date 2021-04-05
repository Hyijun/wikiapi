use std::collections::HashMap;
use crate::page::Page;
use crate::user::User;
use std::sync::Arc;

#[derive(Clone)]
pub struct PagesDepot {
    pub pages: Arc<HashMap<String, Page>>
}

impl PagesDepot {
    pub fn new() -> PagesDepot {
        PagesDepot { pages: Arc::new(HashMap::new()) }
    }

    pub fn from_map(hash_map: HashMap<String, Page>) -> PagesDepot {
        PagesDepot { pages: Arc::new(hash_map) }
    }

    pub fn get_amount(&self) -> usize {
        self.pages.len()
    }
}

#[derive(Clone)]
pub struct UsersDepot {
    pub users: Arc<HashMap<String, User>>
}


impl UsersDepot {
    pub fn new() -> UsersDepot {
        UsersDepot { users: Arc::new(HashMap::new()) }
    }

    pub fn from_map(hash_map: HashMap<String, User>) -> UsersDepot {
        UsersDepot { users: Arc::new(hash_map) }
    }

    pub fn get_amount(&self) -> usize {
        self.users.len()
    }
}
