struct Solution {}

// O(n) time. O(n) space. Stack.
impl Solution {
    pub fn decode_string(s: String) -> String {
        let s = s.as_bytes();
        let mut stack = vec![];
        let mut buf = Box::new(vec![]);
        let mut k = 0;

        for c in s {
            if c.is_ascii_digit() {
                k = k * 10 + (c - b'0') as usize;
            } else if *c == b'[' {
                stack.push((buf, k));
                buf = Box::new(vec![]);
                k = 0;
            } else if *c == b']' {
                let (mut prev_buf, prev_k) = stack.pop().unwrap();
                for _ in 0..prev_k {
                    prev_buf.extend_from_slice(&buf);
                }
                buf = prev_buf;
                k = 0;
            } else {
                buf.push(*c);
            }
        }

        String::from_utf8(*buf).unwrap()
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
            let actual = Solution::decode_string(case.args.s);
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
        expected: String,
    }

    #[allow(non_snake_case)]
    #[derive(Serialize, Deserialize)]
    struct Args {
        s: String,
    }
}
