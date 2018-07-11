//! Misc utility functions
//!

use toml;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use failure::{Error, err_msg};

fn read_file_to_string(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn parse_toml(path: &Path) -> Result<HashMap<Vec<String>, toml::Value>, Error> {
    let string_content = match read_file_to_string(&path) {
        Ok(val) => val,
        Err(msg) => {
            println!("Error reading {} to string", path.display());
            return Err(msg)
        }
    };

    match string_content.parse() {
        Ok(toml) => {
            let mut collection = HashMap::<Vec<String>, toml::Value>::new();
            let mut key = Vec::<String>::new();
            Ok(parse_to_inner(collection, key, toml))
        },
        Err(msg) => {
            println!("Error parsing {} from string to toml", path.display());
            return Err(err_msg(msg))
        }
    }

}

/// Parse the toml input into the innermost level
fn parse_to_inner(
    mut collection: HashMap<Vec<String>, toml::Value>,
    key: Vec<String>,
    toml_val: toml::Value,
) -> HashMap<Vec<String>, toml::Value> {
    let updated_collection = match toml_val {
        toml::Value::String(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Integer(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Float(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Boolean(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Array(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Datetime(_) => {
            collection.insert(key, toml_val);
            collection
        },
        toml::Value::Table(map) => {
            let mut key = key.clone();
            for (k, v) in map.into_iter() {
                key.push(k);
                collection = parse_to_inner(collection, key.clone(), v);
                key.pop();
            }
            collection
        },
    };
    updated_collection
}
