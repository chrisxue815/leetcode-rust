struct Solution {}

// O(n^2) time. O(1) space. Array.
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows <= 0 {
            return vec![];
        }

        let num_rows = num_rows as usize;

        let mut triangle: Vec<Vec<i32>> = (0..num_rows).map(|r| vec![0; r + 1]).collect();
        triangle[0][0] = 1;

        for r in 1..num_rows {
            triangle[r][0] = 1;
            triangle[r][r] = 1;

            for c in 1..r {
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
            let actual = Solution::generate(case.args.numRows);
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

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        numRows: i32,
    }
}
