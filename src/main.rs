//! Display the diff between two toml files
//!

extern crate clap;
extern crate toml;
extern crate failure;

use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use failure::{Error, err_msg};
use clap::{Arg, App};

fn input_args() -> Result<(PathBuf, PathBuf, bool), Error> {
    let matches = App::new("difftoml")
                    .version("0.1.0")
                    .author("Ole-Johan Skrede")
                    .about("Diplay the difference between two toml files")
                    .arg(Arg::with_name("first")
                        .value_name("TOML FILE")
                        .help("First toml file")
                        .takes_value(true)
                        .required(true)
                    )
                    .arg(Arg::with_name("second")
                        .value_name("TOML FILE")
                        .help("Second toml file")
                        .takes_value(true)
                        .required(true)
                    )
                    .arg(Arg::with_name("display_equal")
                        .short("d")
                        .long("display_equal")
                        .help("Toggle this if you want to display the value of entries that \
                               are equal in the two files.")
                        .takes_value(false)
                    )
                    .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let first_path = Path::new(matches.value_of("first").unwrap_or(""));
    let second_path = Path::new(matches.value_of("second").unwrap_or(""));
    let display_equal = matches.is_present("display_equal");

    if !first_path.exists() {
        return Err(err_msg(format!("Path does not exist: {}", first_path.display())))
    }
    if !second_path.exists() {
        return Err(err_msg(format!("Path does not exist: {}", second_path.display())))
    }

    if first_path.extension().unwrap_or(OsStr::new("")) != "toml" {
        return Err(err_msg(format!("Path is not a toml file: {}", first_path.display())))
    }
    if second_path.extension().unwrap_or(OsStr::new("")) != "toml" {
        return Err(err_msg(format!("Path is not a toml file: {}", second_path.display())))
    }

    Ok((first_path.to_path_buf(), second_path.to_path_buf(), display_equal))
}

fn read_file_to_string(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    Ok(buffer)
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

fn parse_toml(path: &Path) -> Result<HashMap<Vec<String>, toml::Value>, Error> {
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

fn compare_vectors<T: Eq+Clone>(
    first: &Vec<T>,
    second: &Vec<T>
) -> Result<(Vec<T>, Vec<T>, Vec<T>), Error> {
    let mut in_first_only = Vec::<T>::new();
    let mut in_second_only = Vec::<T>::new();
    let mut in_both = Vec::<T>::new();

    for element in first {
        if second.contains(&element) {
            in_both.push(element.clone());
        } else {
            in_first_only.push(element.clone());
        }
    }

    let mut in_both_wrt_second = Vec::<T>::new();
    for element in second {
        if first.contains(&element) {
            in_both_wrt_second.push(element.clone());
        } else {
            in_second_only.push(element.clone());
        }
    }

    let mut not_found = false;
    for element in in_both.iter() {
        if !in_both_wrt_second.contains(&element) {
            not_found = true;
        }
    }
    for element in in_both_wrt_second.iter() {
        if !in_both.contains(&element) {
            not_found = true;
        }
    }

    if not_found {
        return Err(err_msg("ERROR: Asymmetric comparison"))
    }

    Ok((in_first_only, in_second_only, in_both))
}

fn display(
    first_path: &Path,
    second_path: &Path,
    first_collection: &HashMap<Vec<String>, toml::Value>,
    second_collection: &HashMap<Vec<String>, toml::Value>,
    keys_in_first_only: Vec<&Vec<String>>,
    keys_in_second_only: Vec<&Vec<String>>,
    keys_in_both: Vec<&Vec<String>>,
    display_equal: bool,
) {
    if !keys_in_first_only.is_empty() {
        println!("");
        println!("Entries only found in {}", first_path.display());
        for key in keys_in_first_only {
            match first_collection.get(key) {
                Some(val) => {
                    println!("{:?}: {}", key, val);
                },
                None => unreachable!(),
            }
        }
    }

    if !keys_in_second_only.is_empty() {
        println!("");
        println!("Entries only found in {}", second_path.display());
        for key in keys_in_second_only {
            match second_collection.get(key) {
                Some(val) => {
                    println!("{:?}: {}", key, val);
                },
                None => unreachable!(),
            }
        }
    }

    if !keys_in_both.is_empty() {
        println!("");
        for key in keys_in_both.clone() {
            let first_val = match first_collection.get(key) {
                Some(val) => val,
                None => unreachable!(),
            };
            let second_val = match second_collection.get(key) {
                Some(val) => val,
                None => unreachable!(),
            };
            if first_val != second_val {
                println!("Unequal value for key {:?}", key);
                println!("<: {}", first_val);
                println!(">: {}", second_val);
            }
        }

        if display_equal {
            println!("");
            for key in keys_in_both {
                let first_val = match first_collection.get(key) {
                    Some(val) => val,
                    None => unreachable!(),
                };
                let second_val = match second_collection.get(key) {
                    Some(val) => val,
                    None => unreachable!(),
                };
                if first_val == second_val {
                    println!("Equal value for key {:?}", key);
                    println!("<: {}", first_val);
                    println!(">: {}", second_val);
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {

    let (first_path, second_path, display_equal) = input_args()?;

    let first_collection = parse_toml(&first_path)?;
    let second_collection = parse_toml(&second_path)?;

    let first_keys: Vec<&Vec<String>> = first_collection.keys().collect();
    let second_keys: Vec<&Vec<String>> = second_collection.keys().collect();

    let (keys_in_first_only, keys_in_second_only, keys_in_both) = compare_vectors(&first_keys,
                                                                                  &second_keys)?;
    display(
        &first_path,
        &second_path,
        &first_collection,
        &second_collection,
        keys_in_first_only,
        keys_in_second_only,
        keys_in_both,
        display_equal,
    );


    Ok(())
}
