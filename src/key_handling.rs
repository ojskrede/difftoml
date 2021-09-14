//! Misc utility functions regarding key handling
//!

use anyhow::{anyhow, Error};
use itertools::Itertools;

pub type Key = Vec<String>;

/// Contains the result of a key-comparison between two files
pub struct KeyOrigins<T: Eq + Clone> {
    /// Keys that are only in the first file
    first_only: Vec<T>,
    /// Keys that are only in the second file
    second_only: Vec<T>,
    /// Keys that are in both files
    both: Vec<T>,
}

impl<T: Eq + Clone> KeyOrigins<T> {
    fn new(first_only: &[T], second_only: &[T], both: &[T]) -> Self {
        KeyOrigins {
            first_only: first_only.to_vec(),
            second_only: second_only.to_vec(),
            both: both.to_vec(),
        }
    }

    pub fn first_only(&self) -> Vec<T> {
        self.first_only.clone()
    }

    pub fn second_only(&self) -> Vec<T> {
        self.second_only.clone()
    }

    pub fn both(&self) -> Vec<T> {
        self.both.clone()
    }
}

/// Compares two vectors by partitioning the elements in the two into three new vectors
///
/// The contents of the three vectors are
///
/// 1: Elements only in the first vector
/// 2: Elements only in the second vector
/// 3: Elements only in the third vector
///
pub fn compare_vectors<T: Eq + Clone>(first: &[T], second: &[T]) -> Result<KeyOrigins<T>, Error> {
    let mut in_first_only = Vec::<T>::new();
    let mut in_second_only = Vec::<T>::new();
    let mut in_both = Vec::<T>::new();

    for element in first {
        if second.contains(element) {
            in_both.push(element.clone());
        } else {
            in_first_only.push(element.clone());
        }
    }

    let mut in_both_wrt_second = Vec::<T>::new();
    for element in second {
        if first.contains(element) {
            in_both_wrt_second.push(element.clone());
        } else {
            in_second_only.push(element.clone());
        }
    }

    let mut not_found = false;
    for element in in_both.iter() {
        if !in_both_wrt_second.contains(element) {
            not_found = true;
        }
    }
    for element in in_both_wrt_second.iter() {
        if !in_both.contains(element) {
            not_found = true;
        }
    }

    if not_found {
        return Err(anyhow!("ERROR: Asymmetric comparison"));
    }

    Ok(KeyOrigins::new(&in_first_only, &in_second_only, &in_both))
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
pub fn filter_keys(keys: &[Key], blackstr: Option<String>) -> Vec<Vec<String>> {
    let mut included_keys = Vec::<Key>::new();

    match blackstr {
        Some(val) => {
            let blacklist: Vec<String> = val.split(',').map(String::from).collect();

            for key in keys.iter() {
                let mut include_key = true;
                let key_str = key.iter().join(".");
                for blacklisted_key in blacklist.iter() {
                    if key_str.contains(blacklisted_key) {
                        include_key = false;
                    }
                }
                if include_key {
                    included_keys.push(key.to_vec());
                }
            }
        }
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
            Ok(result) => {
                assert_eq!(vec![1, 2, 3], result.first_only());
                assert_eq!(vec![7, 8, 9], result.second_only());
                assert_eq!(vec![4, 5, 6], result.both());
            }
            Err(_) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_filter_keys_1() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = None;
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_2() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("key1"));
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_3() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("key3"));
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key1")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_4() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("key"));
        let test = filter_keys(&keys, blackstr);
        let correct = Vec::<Vec<String>>::new();
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_5() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("ke.y1"));
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_6() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key2"),
                String::from("key3"),
                String::from("key4"),
            ],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("key2.key3"));
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key1")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }

    #[test]
    fn test_filter_keys_7() {
        let keys = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key2"),
                String::from("key3"),
                String::from("key4"),
            ],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        let blackstr = Some(String::from("key2.key3.key4"));
        let test = filter_keys(&keys, blackstr);
        let correct = vec![
            vec![String::from("key1")],
            vec![String::from("key2"), String::from("key3")],
            vec![
                String::from("key4"),
                String::from("key5"),
                String::from("key6"),
            ],
        ];
        assert_eq!(correct, test);
    }
}
