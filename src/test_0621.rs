struct Solution {}

// O(n) time. O(1) space. Math, array.
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let len = tasks.len();
        let mut counts = vec![0; 26];
        let mut max_count = 0;
        let mut max_count_freq = 0;

        for task in tasks {
            let index = (task as u8 - b'A') as usize;
            counts[index] += 1;
            if max_count == counts[index] {
                max_count_freq += 1;
            } else if max_count < counts[index] {
                max_count = counts[index];
                max_count_freq = 1;
            }
        }

        std::cmp::max(len as i32, (max_count - 1) * (n + 1) + max_count_freq)
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
            let actual = Solution::least_interval(case.args.tasks, case.args.n);
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
        tasks: Vec<char>,
        n: i32,
    }
}
