struct Solution {}

// O(n) time. O(1) space.
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let ones: i32 = students.iter().sum();
        let zeros = students.len() as i32 - ones;
        let mut students = vec![zeros, ones];

        for (i, sandwich) in sandwiches.iter().enumerate() {
            let sandwich = (*sandwich) as usize;
            if students[sandwich] <= 0 {
                return (sandwiches.len() - i) as i32;
            }
            students[sandwich] -= 1;
        }

        0
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
            let actual = Solution::count_students(case.args.students, case.args.sandwiches);
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
        students: Vec<i32>,
        sandwiches: Vec<i32>,
    }
}
