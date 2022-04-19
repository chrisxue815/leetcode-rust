struct Solution {}

// O(n) time. O(1) space.
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut sum = 0;

        for num in nums {
            if sum <= 0 {
                sum = num;
            } else {
                sum += num;
            }
            if result < sum {
                result = sum;
            }
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
            let actual = Solution::max_sub_array(case.args.nums);
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
    }
}
