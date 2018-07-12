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
///
/// toml::Value is an enum
///
/// ```rust,no-run
/// pub enum Value {
///     String(String),
///     Integer(i64),
///     Float(f64),
///     Boolean(bool),
///     Datetime(Datetime),
///     Array(Array),
///     Table(Table),
/// }
/// ```
///
/// The toml structure is
///
/// ```text,no-run
/// lvl0_key0 = value
/// lvl0_key1 = value
/// lvl0_key2
///     lvl1_key0 = value
///     lvl1_key1
///         lvl2_key0 = value
///         lvl2_key1 = value
/// lvl0_key3
///     lvl1_key0 = value
///     lvl1_key1 = value
/// ```
///
/// where every key followed by a new level holds a Value::Table, and every "key = value" holds one
/// of the other variants. This function untangles all Value::Table variants to one of the other
/// variants. As an example, the result of the above structure should be a hashmap looking like
/// this (sorted for ease of reading), where toml::InnerValue a subset of toml::Value excluding
/// toml::Value::Table
///
/// ```
/// {
///     ["lvl0_key0"]: toml::InnerValue,
///     ["lvl0_key1"]: toml::InnerValue,
///     ["lvl0_key2", "lvl1_key0"]: toml::InnerValue,
///     ["lvl0_key2", "lvl1_key1", "lvl2_key0"]: toml::InnerValue,
///     ["lvl0_key2", "lvl1_key1", "lvl2_key1"]: toml::InnerValue,
///     ["lvl0_key3", "lvl1_key0"]: toml::InnerValue,
///     ["lvl0_key3", "lvl1_key1"]: toml::InnerValue,
/// }
/// ```
///
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


#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_parse_to_inner() {
        let toml_str = r#"
            lvl0_key0 = "Hello world"
            lvl0_key1 = 123

            [lvl0_key2]
            lvl1_key0 = 1.23

            [lvl0_key2.lvl1_key1]
            lvl2_key0 = true
            lvl2_key1 = 1979-05-27T07:32:00Z

            [lvl0_key3]
            lvl1_key0 = [123, 456, 789]
            lvl1_key1 = ["first", "second", "third"]
        "#;
        match toml_str.parse() {
            Ok(toml) => {
                let mut test_collection = HashMap::<Vec<String>, toml::Value>::new();
                let mut key = Vec::<String>::new();
                let test_collection = parse_to_inner(test_collection, key, toml);
                let mut true_collection = HashMap::new();
                true_collection.insert(vec![String::from("lvl0_key0")],
                                       toml::Value::String(String::from("Hello world")));
                true_collection.insert(vec![String::from("lvl0_key1")],
                                       toml::Value::Integer(123));
                true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key0")],
                                       toml::Value::Float(1.23));
                true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key1"), String::from("lvl2_key0")],
                                       toml::Value::Boolean(true));
                let datetime = toml::value::Datetime::from_str("1979-05-27T07:32:00Z")
                                                        .expect("Could not create datetime");
                true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key1"), String::from("lvl2_key1")],
                                       toml::Value::Datetime(datetime));
                true_collection.insert(vec![String::from("lvl0_key3"), String::from("lvl1_key0")],
                                       toml::Value::Array(vec![
                                            toml::Value::Integer(123),
                                            toml::Value::Integer(456),
                                            toml::Value::Integer(789),
                                       ]));
                true_collection.insert(vec![String::from("lvl0_key3"), String::from("lvl1_key1")],
                                       toml::Value::Array(vec![
                                            toml::Value::String(String::from("first")),
                                            toml::Value::String(String::from("second")),
                                            toml::Value::String(String::from("third")),
                                       ]));

                assert_eq!(true_collection, test_collection)
            },
            Err(msg) => {
                println!("Error parsing string to toml");
                println!("{:?}", msg);
                assert!(false)
            }
        }
    }

    #[test]
    fn test_parse_toml() {
        let path = Path::new("assets/test_3.toml");
        let test_collection = parse_toml(&path).expect("Could not parse toml");

        let mut true_collection = HashMap::new();
        true_collection.insert(vec![String::from("lvl0_key0")],
                               toml::Value::String(String::from("Hello world")));
        true_collection.insert(vec![String::from("lvl0_key1")],
                               toml::Value::Integer(123));
        true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key0")],
                               toml::Value::Float(1.23));
        true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key1"), String::from("lvl2_key0")],
                               toml::Value::Boolean(true));
        let datetime = toml::value::Datetime::from_str("1979-05-27T07:32:00Z")
                                                .expect("Could not create datetime");
        true_collection.insert(vec![String::from("lvl0_key2"), String::from("lvl1_key1"), String::from("lvl2_key1")],
                               toml::Value::Datetime(datetime));
        true_collection.insert(vec![String::from("lvl0_key3"), String::from("lvl1_key0")],
                               toml::Value::Array(vec![
                                    toml::Value::Integer(123),
                                    toml::Value::Integer(456),
                                    toml::Value::Integer(789),
                               ]));
        true_collection.insert(vec![String::from("lvl0_key3"), String::from("lvl1_key1")],
                               toml::Value::Array(vec![
                                    toml::Value::String(String::from("first")),
                                    toml::Value::String(String::from("second")),
                                    toml::Value::String(String::from("third")),
                               ]));

        assert_eq!(true_collection, test_collection)
    }
}
