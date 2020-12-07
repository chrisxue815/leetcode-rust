struct Solution {}

// O(mn) time. O(mn) space. DP.
impl Solution {
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

        for i in 1..=a.len() {
            for j in 1..=b.len() {
                if a[i - 1] == b[j - 1] {
                    let curr = dp[i - 1][j - 1] + 1;
                    dp[i][j] = curr;
                    if result < curr {
                        result = curr
                    }
                }
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
            let actual = Solution::find_length(case.args.A, case.args.B);
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
        A: Vec<i32>,
        B: Vec<i32>,
    }
}
