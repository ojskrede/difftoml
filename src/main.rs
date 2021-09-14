//! Display the diff between two toml files
//!
use anyhow::{anyhow, Error};
use clap::{App, Arg};
use colored::Colorize;
use itertools::Itertools;
use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

mod key_handling;
mod parse;

use key_handling::{Key, KeyOrigins};

fn input_args() -> Result<(PathBuf, PathBuf, bool, bool, Option<String>), Error> {
    let matches = App::new("difftoml")
        .version("0.2.0")
        .author("Ole-Johan Skrede")
        .about("Diplay the difference between two toml files")
        .arg(
            Arg::with_name("first")
                .value_name("TOML FILE")
                .help("First toml file")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("second")
                .value_name("TOML FILE")
                .help("Second toml file")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("exclude")
                .short("x")
                .long("exclude")
                .value_name("KEY LIST")
                .help("Key(s) to ignore in the diff")
                .long_help(
                "Specify a single key or a list of keys that you want to exclude in the diff. \n\
                Use a comma mark ',' (without whitespace) to distinguish keys. Use a \n\
                period mark '.' (without whitespace) to describe key-level hierarchy \n\
                Usage: \n\
                \t -x key1  // Excludes all entries which has 'key1' as a key somewhere in \n\
                \t          // its key hierarchy. E.g. 'key1' or 'key0.key1.key2' or \n\
                \t          // 'containskey1inside', but not key0.ke.y1key2'. \n\
                \t -x key1.key2  // Excludes all entries which has 'key2' directly after 'key1' \n\
                \t               // somewhere in its key hierarchy. E.g. 'key1.key2' or \n\
                \t               // 'key0.key1.key2' but not 'key0.key1.key3.key2'. \n\
                \t -x key1,key2.key3 // A union of the above two behaviours.")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("display_equal")
                .short("e")
                .long("display_equal")
                .help("Toggle this if you want to display the value of entries that \
                       are equal in the two files.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Toggle this if you want colored output")
                .takes_value(false)
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let first_path = Path::new(matches.value_of("first").unwrap_or(""));
    let second_path = Path::new(matches.value_of("second").unwrap_or(""));
    let display_equal = matches.is_present("display_equal");
    let color = matches.is_present("color");
    let exclude = matches.value_of("exclude").map(String::from);

    if !first_path.exists() {
        return Err(anyhow!("Path does not exist: {}", first_path.display()));
    }
    if !second_path.exists() {
        return Err(anyhow!("Path does not exist: {}", second_path.display()));
    }

    if first_path.extension().unwrap_or_else(|| OsStr::new("")) != "toml" {
        return Err(anyhow!("Path is not a toml file: {}", first_path.display()));
    }
    if second_path.extension().unwrap_or_else(|| OsStr::new("")) != "toml" {
        return Err(anyhow!(
            "Path is not a toml file: {}",
            second_path.display()
        ));
    }

    Ok((
        first_path.to_path_buf(),
        second_path.to_path_buf(),
        display_equal,
        color,
        exclude,
    ))
}

fn display(
    first_path: &Path,
    second_path: &Path,
    first_collection: &HashMap<Vec<String>, toml::Value>,
    second_collection: &HashMap<Vec<String>, toml::Value>,
    key_origins: &KeyOrigins<Key>,
    display_equal: bool,
    color: bool,
) {
    if !key_origins.first_only().is_empty() {
        if color {
            let output = format!("\n{}", first_path.display());
            println!("{}", output.blue());
        } else {
            println!("\nEntries only found in {}", first_path.display());
        }
        for key in key_origins.first_only().iter() {
            match first_collection.get(key) {
                Some(val) => {
                    println!("{}: {}", key.iter().join("."), val);
                }
                None => unreachable!(),
            }
        }
    }

    if !key_origins.second_only().is_empty() {
        if color {
            let output = format!("\n{}", second_path.display());
            println!("{}", output.yellow());
        } else {
            println!("\nEntries only found in {}", second_path.display());
        }
        for key in key_origins.second_only().iter() {
            match second_collection.get(key) {
                Some(val) => {
                    println!("{}: {}", key.iter().join("."), val);
                }
                None => unreachable!(),
            }
        }
    }

    if !key_origins.both().is_empty() {
        for key in key_origins.both().iter() {
            let first_val = match first_collection.get(key) {
                Some(val) => val,
                None => unreachable!(),
            };
            let second_val = match second_collection.get(key) {
                Some(val) => val,
                None => unreachable!(),
            };
            if first_val != second_val {
                if color {
                    let output = key.iter().join(".");
                    println!("\n{}", output.red());
                    println!("{} {}", "<".blue(), first_val);
                    println!("{} {}", ">".yellow(), second_val);
                } else {
                    println!("\nUnequal value for key '{}'", key.iter().join("."));
                    println!("< {}", first_val);
                    println!("> {}", second_val);
                }
            }
        }

        if display_equal {
            for key in key_origins.both().iter() {
                let first_val = match first_collection.get(key) {
                    Some(val) => val,
                    None => unreachable!(),
                };
                let second_val = match second_collection.get(key) {
                    Some(val) => val,
                    None => unreachable!(),
                };
                if first_val == second_val {
                    if color {
                        let output = key.iter().join(".");
                        println!("\n{}", output.green());
                        println!("{} {}", "<".blue(), first_val);
                        println!("{} {}", ">".yellow(), second_val);
                    } else {
                        println!("\nEqual value for key '{}'", key.iter().join("."));
                        println!("< {}", first_val);
                        println!("> {}", second_val);
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let (first_path, second_path, display_equal, color, exclude) = input_args()?;

    let first_collection = parse::parse_toml(&first_path)?;
    let second_collection = parse::parse_toml(&second_path)?;

    let first_keys: Vec<Key> = first_collection.keys().cloned().collect();
    let second_keys: Vec<Key> = second_collection.keys().cloned().collect();

    let first_keys = key_handling::filter_keys(&first_keys, exclude.clone());
    let second_keys = key_handling::filter_keys(&second_keys, exclude);

    let key_origins = key_handling::compare_vectors(&first_keys, &second_keys)?;

    display(
        &first_path,
        &second_path,
        &first_collection,
        &second_collection,
        &key_origins,
        display_equal,
        color,
    );

    Ok(())
}
