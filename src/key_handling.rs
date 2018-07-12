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
/// ["key1", "key2.key3"]
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
pub fn convert_key_list_to_key_str(key_list: &Vec<String>) -> String {
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
fn convert_list_of_keys_to_string_of_keys(list_of_keys: &Vec<Vec<String>>) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_vectors() {
        let v1 = vec![1, 2, 3, 4, 5, 6];
        let v2 = vec![4, 5, 6, 7, 8, 9];

        match compare_vectors(&v1, &v2) {
            Ok((test_1, test_2, test_both)) => {
                assert_eq!(vec![1, 2, 3], test_1);
                assert_eq!(vec![7, 8, 9], test_2);
                assert_eq!(vec![4, 5, 6], test_both);
            },
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_convert_string_of_keys_to_list_of_key_str() {
        let string_of_keys = "key1,key2.key3,key4.key5.key6";
        let test = convert_string_of_keys_to_list_of_key_str(string_of_keys);
        let correct = vec![String::from("key1"),
                           String::from("key2.key3"),
                           String::from("key4.key5.key6")];

        assert_eq!(correct, test);
    }

    #[test]
    fn test_convert_string_of_keys_to_list_of_keys() {
        let string_of_keys = "key1,key2.key3,key4.key5.key6";
        let test = convert_string_of_keys_to_list_of_keys(string_of_keys);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key2"), String::from("key3")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_convert_key_list_to_key_str() {
        let key_list = vec![String::from("key1"),
                            String::from("key2"),
                            String::from("key3")];
        let test = convert_key_list_to_key_str(&key_list);
        let correct = "key1.key2.key3";
        assert_eq!(correct, test);
    }

    #[test]
    fn test_convert_list_of_keys_to_string_of_keys() {
        let list_of_keys =
            vec![vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let test = convert_list_of_keys_to_string_of_keys(&list_of_keys);
        let correct = "key1,key2.key3,key4.key5.key6";
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_1() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = None;
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key2"), String::from("key3")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_2() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("key1"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key2"), String::from("key3")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_3() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("key3"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_4() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("key"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = Vec::<Vec<String>>::new();
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_5() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("ke.y1"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key2"), String::from("key3")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_6() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key2"), String::from("key3"), String::from("key4")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("key2.key3"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_7() {
        let keys = vec![vec![String::from("key1")],
                        vec![String::from("key2"), String::from("key3")],
                        vec![String::from("key2"), String::from("key3"), String::from("key4")],
                        vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        let blackstr = Some(String::from("key2.key3.key4"));
        let test = filter_keys(&keys.iter().collect(), blackstr);
        let correct = vec![vec![String::from("key1")],
                           vec![String::from("key2"), String::from("key3")],
                           vec![String::from("key4"), String::from("key5"), String::from("key6")]];
        assert_eq!(correct, test);
    }

}
