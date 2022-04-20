struct Solution {}

// O(n) time. O(1) space. Manual parsing.
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut sum = 0;
        let mut sign = 1;
        let (mut product, mut i) = read_number(s, 0);

        while i < len {
            while i < len && s[i] == b' ' {
                i += 1;
            }

            if i >= len {
                break;
            }

            let op = s[i];
            i += 1;

            let (num, j) = read_number(s, i);
            i = j;

            match op {
                b'+' => {
                    sum += product * sign;
                    sign = 1;
                    product = num;
                }
                b'-' => {
                    sum += product * sign;
                    sign = -1;
                    product = num;
                }
                b'*' => {
                    product *= num;
                }
                b'/' => {
                    product /= num;
                }
                _ => {}
            }
        }

        sum + product * sign
    }
}

fn read_number(s: &[u8], mut i: usize) -> (i32, usize) {
    while i < s.len() && s[i] == b' ' {
        i += 1;
    }

    let mut num: i32 = 0;

    while i < s.len() {
        let c = s[i];
        if c < b'0' || c > b'9' {
            break;
        }
        num *= 10;
        num += (s[i] - b'0') as i32;
        i += 1;
    }

    (num, i)
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
