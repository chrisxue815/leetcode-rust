use std::fs;
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;
use serde::de::DeserializeOwned;

pub fn load_test_json<T: DeserializeOwned>(solution_path: &str) -> T {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(test_\d+)").unwrap();
    }

    let solution_path = Path::new(solution_path);
    let test_name = solution_path.file_stem().unwrap().to_str().unwrap();

    let cap = RE.captures(test_name).unwrap();

    let file = format!("leetcode_test_cases/{}.json", &cap[0]);
    let json_text = fs::read_to_string(file).unwrap();
    serde_json::from_str(json_text.as_str()).unwrap()
}
