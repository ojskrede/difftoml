//! Misc utility functions regarding key handling
//!

use failure::{Error, err_msg};

/// Compares two vectors by partitioning the elements in the two into three new vectors
///
/// The contents of the three vectors are
///
/// 1: Elements only in the first vector
/// 2: Elements only in the second vector
/// 3: Elements only in the third vector
///
pub fn compare_vectors<T: Eq+Clone>(
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

/// Converts a string of the form
///
/// "key1,key2.key3"
///
/// to a list of the form
///
/// [["key1"], ["key2.key3"]]
fn convert_string_of_keys_to_list_of_key_str(string_of_keys: &str) -> Vec<String> {
    let mut list_of_keys = Vec::<String>::new();

    for key in string_of_keys.split(",") {
        //let key_list: Vec<String> = key.split(".").map(|x| String::from(x)).collect();
        list_of_keys.push(String::from(key));
    }

    list_of_keys
}

/// Converts a string of the form
///
/// "key1,key2.key3"
///
/// to a list of the form
///
/// [["key1"], ["key2", "key3"]]
#[allow(dead_code)]
fn convert_string_of_keys_to_list_of_keys(string_of_keys: &str) -> Vec<Vec<String>> {
    let mut list_of_keys = Vec::<Vec<String>>::new();

    for key in string_of_keys.split(",") {
        let key_list: Vec<String> = key.split(".").map(|x| String::from(x)).collect();
        list_of_keys.push(key_list);
    }

    list_of_keys
}

/// Converst a list of the form
///
/// ["key1", "key2", "key3"]
///
/// to a string of the form
///
/// "key1.key2.key3"
fn convert_key_list_to_key_str(key_list: &Vec<String>) -> String {
    let mut key_str = String::from("");
    for (ind, subkey) in key_list.iter().enumerate() {
        key_str.push_str(subkey);
        if ind < key_list.len() - 1 {
            key_str.push_str(".");
        }
    }
    key_str
}

/// Converts a list of the form
///
/// [["key1"], ["key2", "key3"]]
///
/// to a string of the form
///
/// "key1,key2.key3"
#[allow(dead_code)]
fn convert_list_of_keys_to_string_of_keys(list_of_keys: Vec<Vec<String>>) -> String {
    let mut string_of_keys = String::from("");

    for (ind, key) in list_of_keys.iter().enumerate() {
        string_of_keys.push_str(&convert_key_list_to_key_str(key));
        if ind < list_of_keys.len() - 1 {
            string_of_keys.push_str(",");
        }
    }
    string_of_keys
}

/// Exclude keys from the input key list.
///
/// keys is a vector that can look something like this
///
/// [["key1"], ["key2", "key3"]]
///
/// meaning that "key1" is a key on the first level in the toml file, and "key3" is a key on the
/// second level of the toml file under the key "key2", written "key2.key3".
///
/// blackstr is a string with keys to exclude from the keys list. The string should look something
/// like this
///
/// "key1,key2.key3"
///
/// which is interpreted to be equal to the above keys example.
///
/// This function filters every entry that has one (or more) of the exclude keys as part of its
/// key.
pub fn filter_keys(keys: &Vec<&Vec<String>>, blackstr: Option<String>) -> Vec<Vec<String>> {
    let mut included_keys = Vec::<Vec<String>>::new();

    match blackstr {
        Some(val) => {
            let blacklist = convert_string_of_keys_to_list_of_key_str(&val);

            for key in keys.iter() {
                let mut include_key = true;
                let key_str = convert_key_list_to_key_str(key); // ["key1", "key2"] -> "key1.key2"
                for blacklisted_key in blacklist.iter() {
                    if key_str.contains(blacklisted_key) {
                        include_key = false;
                    }
                }
                if include_key {
                    included_keys.push(key.to_vec());
                }
            }
        },
        None => {
            for key in keys.iter() {
                included_keys.push(key.to_vec());
            }
        }
    }
    included_keys
}
