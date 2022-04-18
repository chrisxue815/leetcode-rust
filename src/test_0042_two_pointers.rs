struct Solution {}

// O(n) time. O(1) space. Two pointers.
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut result = 0;
        let mut lo = 0;
        let mut hi = height.len() - 1;
        let mut lo_max = height[0];
        let mut hi_max = height[hi];

        while lo < hi {
            if lo_max <= hi_max {
                result += lo_max - height[lo];
                lo += 1;
                if lo_max < height[lo] {
                    lo_max = height[lo];
                }
            } else {
                result += hi_max - height[hi];
                hi -= 1;
                if hi_max < height[hi] {
                    hi_max = height[hi];
                }
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
            let actual = Solution::trap(case.args.height);
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
        height: Vec<i32>,
    }
}
