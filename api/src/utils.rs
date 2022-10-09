extern crate dotenv;

use dotenv::dotenv;
use std::{
    collections::HashMap,
    env::{self},
};

#[derive(Debug)]
pub struct MyVars {}

// Takes in a string with the key value to find and returns the
// Environment variable if it is found.
pub fn get_app_vars(key_wanted: String) -> String {
    dotenv().ok();

    let mut var_hash = HashMap::new();

    for (key, value) in env::vars() {
        var_hash.insert(key, value);
    }
    match var_hash.get(&key_wanted.to_owned()) {
        Some(var) => var.to_owned(),
        None => panic!("No Env variable with key = {}", &key_wanted.to_owned()),
    }
}
