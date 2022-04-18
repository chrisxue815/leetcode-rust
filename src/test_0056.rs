struct Solution {}

// O(nlog(n)) time. O(1) space. Interval, sorting by start in ascending order, greedy.
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        intervals.sort_unstable_by_key(|x| x[0]);

        for interval in intervals {
            if let Some(last) = result.last_mut() {
                if interval[0] <= last[1] {
                    if last[1] < interval[1] {
                        last[1] = interval[1];
                    }
                    continue;
                }
            }
            result.push(interval);
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
            let actual = Solution::merge(case.args.intervals);
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
        intervals: Vec<Vec<i32>>,
    }
}
