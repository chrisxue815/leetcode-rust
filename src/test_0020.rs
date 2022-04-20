struct Solution {}

// O(n) time. O(n) space. Stack.
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for c in s.as_bytes() {
            match c {
                b'(' => stack.push(b'('),
                b'[' => stack.push(b'['),
                b'{' => stack.push(b'{'),
                b')' => {
                    if let Some(last) = stack.pop() {
                        if last != b'(' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                b']' => {
                    if let Some(last) = stack.pop() {
                        if last != b'[' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                b'}' => {
                    if let Some(last) = stack.pop() {
                        if last != b'{' {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => { return false; }
            }
        }

        stack.is_empty()
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
            let actual = Solution::is_valid(case.args.s);
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
        expected: bool,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        s: String,
    }
}
