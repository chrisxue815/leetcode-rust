struct Solution {}

// O(n) time. O(n) space. Stack.
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut sum = 0;
        let mut stack = vec![];
        let mut num = 0;
        let mut sign = 1;

        for c in s.as_bytes() {
            match c {
                b'+' | b'-' => {
                    sum += num * sign;
                    num = 0;
                    sign = 44 - *c as i32;
                }
                b'(' => {
                    stack.push((sum, sign));
                    sum = 0;
                    sign = 1;
                }
                b')' => {
                    let product = sum + num * sign;
                    num = 0;
                    let (prev_sum, prev_sign) = stack.pop().unwrap();
                    sum = prev_sum;
                    sign = prev_sign;
                    sum += product * sign;
                }
                _ => {
                    if c.is_ascii_digit() {
                        num = num * 10 + (c - b'0') as i32;
                    }
                }
            }
        }

        sum + num * sign
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
            let actual = Solution::calculate(case.args.s);
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
        s: String,
    }
}
