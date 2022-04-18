struct Solution {}

// O(n) time. O(1) space.
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut lowest_so_far = i32::MAX;

        for price in prices {
            let profit = price - lowest_so_far;
            if result < profit {
                result = profit;
            }
            if lowest_so_far > price {
                lowest_so_far = price;
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
            let actual = Solution::max_profit(case.args.prices);
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
        prices: Vec<i32>,
    }
}
