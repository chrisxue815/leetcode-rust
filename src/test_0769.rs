struct Solution {}

// O(n) time. O(1) space. Array.
impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut bound = 0;

        for (i, num) in arr.iter().enumerate() {
            let num = *num as usize;
            if bound < num {
                bound = num;
            }
            if i == bound {
                result += 1;
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
            let actual = Solution::max_chunks_to_sorted(case.args.arr);
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
        expected: i32,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        arr: Vec<i32>,
    }
}
