struct Solution {}

// O(n^2) time. O(1) space. Array.
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows <= 0 {
            return vec![];
        }

        let mut triangle: Vec<Vec<i32>> =
            (0..num_rows).map(|r| vec![1; (r + 1) as usize]).collect();

        for r in 2..num_rows {
            let r = r as usize;
            for c in 1..r {
                let c = c as usize;
                triangle[r][c] = triangle[r - 1][c - 1] + triangle[r - 1][c];
            }
        }

        triangle
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
            let actual = Solution::generate(case.args.num_rows);
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
        expected: Vec<Vec<i32>>,
    }

    #[derive(Serialize, Deserialize)]
    struct Args {
        num_rows: i32,
    }
}
