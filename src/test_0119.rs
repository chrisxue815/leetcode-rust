struct Solution {}

// O(n^2) time. O(1) space. Array.
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index < 0 {
            return vec![];
        }

        let row_index = row_index as usize;

        let mut result = vec![1; row_index + 1];

        for r in 2..=row_index {
            for c in (1..r).rev() {
                result[c] += result[c - 1];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use serde::{Deserialize, Serialize};

    #[test]
    fn it_works() {
        let test_data = util::load_test_json::<TestJson>(file!());

        for case in test_data.test_cases {
            let msg = serde_json::to_string(&case.args).unwrap();
            let actual = Solution::get_row(case.args.rowIndex);
            assert_eq!(actual, case.expected, "{}", msg);
        }
    }

    #[derive(Serialize, Deserialize)]
    struct TestJson {
        test_cases: Vec<TestCase>,
    }

    #[derive(Serialize, Deserialize)]
    struct TestCase {
        args: Args,
        expected: Vec<i32>,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        rowIndex: i32,
    }
}
