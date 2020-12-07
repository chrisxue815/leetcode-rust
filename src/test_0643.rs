struct Solution {}

// O(n) time. O(1) space. Array.
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut sum: i32 = (&nums[0..k]).iter().sum();
        let mut max_sum = sum;

        for i in k..nums.len() {
            sum += nums[i] - nums[i - k];
            if max_sum < sum {
                max_sum = sum;
            }
        }

        max_sum as f64 / k as f64
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
            let actual = Solution::find_max_average(case.args.nums, case.args.k);
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
        expected: f64,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        nums: Vec<i32>,
        k: i32,
    }
}
