use std::collections::HashMap;

struct Solution {}

// O(n) time. O(n) space. Hash table, prefix sum.
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut counter = HashMap::from([(0, 1)]);
        let mut sum = 0;

        for num in nums {
            sum += num;
            result += counter.get(&(sum - k)).unwrap_or(&0);
            *counter.entry(sum).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::util;

    use super::*;

    #[test]
    fn it_works() {
        let test_data = util::load_test_json::<TestJson>(file!());

        for case in test_data.test_cases {
            let msg = serde_json::to_string(&case.args).unwrap();
            let actual = Solution::subarray_sum(case.args.nums, case.args.k);
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
        nums: Vec<i32>,
        k: i32,
    }
}
