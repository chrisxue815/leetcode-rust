use std::collections::{HashMap, VecDeque};

struct Solution {}

// O(n) time. O(len(words)) space. Hash table, queue.
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut result = 0;
        let mut queues = HashMap::new();

        for word in &words {
            let mut it = word.bytes();
            let first_char = it.next().unwrap();
            let q = queues.entry(first_char).or_insert_with(|| VecDeque::new());
            q.push_back(it);
        }

        for c in s.bytes() {
            //TODO: is there a way to use get_mut() instead of remove()?
            let mut q = match queues.remove(&c) {
                None => continue,
                Some(x) => x,
            };

            for _ in 0..q.len() {
                let mut it = q.pop_front().unwrap();
                match it.next() {
                    None => result += 1,
                    Some(next_char) => {
                        let q = queues.entry(next_char).or_insert_with(|| VecDeque::new());
                        q.push_back(it);
                    }
                };
            }
        }

        result
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
            let actual = Solution::num_matching_subseq(case.args.S, case.args.words);
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
        S: String,
        words: Vec<String>,
    }
}
