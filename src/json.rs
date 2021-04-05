extern crate serde_json;

use crate::file::read_file;
use crate::file::create_file;
use self::serde_json::{Value, Error};
use std::fs::File;
use std::fmt::Formatter;

pub fn get_json_string(s: &str) -> serde_json::Result<Value> {
    serde_json::from_str(s)
}

pub fn get_json_file(f: File) -> serde_json::Result<Value> {
    serde_json::from_reader(f)
}

pub fn get_value_str(json: serde_json::Value, key: &str) -> String {
    match json[key].as_str() {
        Some(s) => s.to_owned(),
        None => "".to_owned()
    }
}

pub fn get_value_i64(json: serde_json::Value) -> Option<i64> {
    json.as_i64()
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct JsonError { info: String }

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info)
    }
}

impl JsonError {
    pub fn new(info: String) -> JsonError {
        JsonError { info }
    }
}

impl std::error::Error for JsonError {}

