struct Solution {}

// O(n) time. O(1) space.
impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut lo = None;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                lo = Some(i);
                break;
            }
        }

        if let Some(lo) = lo {
            for i in (lo + 1..nums.len()).rev() {
                if nums[i] > nums[lo] {
                    nums.swap(i, lo);
                    nums[lo + 1..].reverse();
                    break;
                }
            }
        } else {
            nums.reverse();
        }
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

        for mut case in test_data.test_cases {
            let msg = serde_json::to_string(&case.args).unwrap();
            Solution::next_permutation(&mut case.args.nums);
            assert_eq!(case.args.nums, case.expected, "{}", msg);
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
        nums: Vec<i32>,
    }
}
