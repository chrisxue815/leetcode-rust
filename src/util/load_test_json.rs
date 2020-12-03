use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

pub fn load_test_json<T: DeserializeOwned>(solution_path: &str) -> T {
    let solution_path = Path::new(solution_path);
    let test_name = solution_path.file_stem().unwrap().to_str().unwrap();
    let file = format!("leetcode_test_cases/{}.json", test_name);
    let json_text = fs::read_to_string(file).unwrap();
    serde_json::from_str(json_text.as_str()).unwrap()
}
