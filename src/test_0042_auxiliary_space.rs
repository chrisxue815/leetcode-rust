struct Solution {}

// O(n) time. O(n) space.
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = Vec::with_capacity(height.len());

        let mut max_so_far = 0;
        for h in &height {
            if max_so_far < *h {
                max_so_far = *h;
            }
            left_max.push(max_so_far);
        }

        let mut result = 0;
        let mut max_so_far = 0;
        for i in (0..height.len()).rev() {
            let h = height[i];
            if max_so_far < h {
                max_so_far = h;
            }

            let bottleneck = std::cmp::min(left_max[i], max_so_far);
            result += bottleneck - h;
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
