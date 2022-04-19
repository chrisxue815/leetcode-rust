struct Solution {}

// O(log(min(m, n))) time. O(1) space. Binary search.
impl Solution {
    pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        let mut m = nums1.len();
        let mut n = nums2.len();

        if m > n {
            std::mem::swap(&mut m, &mut n);
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let mut lo = 0;
        let mut hi = m;
        let half = (m + n + 1) >> 1;

        loop {
            let i = lo + ((hi - lo) >> 1);
            let j = half - i;

            if i > 0 && nums1[i - 1] > nums2[j] {
                hi = i - 1;
            } else if i < m && nums2[j - 1] > nums1[i] {
                lo = i + 1;
            } else {
                let left_max = if i == 0 {
                    nums2[j - 1]
                } else if j == 0 {
                    nums1[i - 1]
                } else {
                    std::cmp::max(nums1[i - 1], nums2[j - 1])
                };

                if (m + n) & 1 == 1 {
                    return left_max as f64;
                }

                let right_min = if i == m {
                    nums2[j]
                } else if j == n {
                    nums1[i]
                } else {
                    std::cmp::min(nums1[i], nums2[j])
                };

                return (left_max + right_min) as f64 / 2.0;
            }
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

        for case in test_data.test_cases {
            let msg = serde_json::to_string(&case.args).unwrap();
            let actual = Solution::find_median_sorted_arrays(case.args.nums1, case.args.nums2);
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
        nums1: Vec<i32>,
        nums2: Vec<i32>,
    }
}
