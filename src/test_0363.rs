struct Solution {}

// Binary search.
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let (width, height, tall) = if rows <= cols {
            (cols, rows, false)
        } else {
            (rows, cols, true)
        };

        let mut result = i32::MIN;

        for hi in 0..height {
            let mut sums = vec![0; width];

            for hj in hi..height {
                for w in 0..width {
                    sums[w] += if tall {
                        matrix[w][hj]
                    } else {
                        matrix[hj][w]
                    };
                }

                let mut curr_sum = 0;
                let mut prev_sum = vec![0, i32::MAX];

                for val in &sums {
                    curr_sum += val;

                    let target = curr_sum - k;
                    let index = bisect_left(&prev_sum, &target);
                    if index < prev_sum.len() - 1 && result < curr_sum - prev_sum[index] {
                        result = curr_sum - prev_sum[index];
                    }

                    let index = bisect_left(&prev_sum, &curr_sum);
                    prev_sum.insert(index, curr_sum);
                }
            }
        }

        result
    }
}

fn bisect_left<T>(a: &[T], x: &T) -> usize
    where T: PartialOrd {
    let mut lo = 0;
    let mut hi = a.len();

    while lo < hi {
        let mid = lo + ((hi - lo) >> 1);
        let mid_val = &a[mid];
        if mid_val < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
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
            let actual = Solution::max_sum_submatrix(case.args.matrix, case.args.k);
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
        matrix: Vec<Vec<i32>>,
        k: i32,
    }
}
