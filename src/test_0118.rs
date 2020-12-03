struct Solution {}

// O(n) time. O(1) space. Array.
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows <= 0 {
            return vec![];
        }

        let mut triangle = Vec::with_capacity(num_rows as usize);
        triangle.push(vec![1]);

        for r in 1..num_rows {
            let mut curr = vec![0; (r + 1) as usize];
            let prev = triangle.last().unwrap();

            curr[0] = prev[0];
            *curr.last_mut().unwrap() = *prev.last().unwrap();

            for c in 1..r {
                curr[c as usize] = prev[(c - 1) as usize] + prev[c as usize];
            }

            triangle.push(curr);
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
