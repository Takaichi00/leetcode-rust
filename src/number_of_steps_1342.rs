// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut count = 0;
        let mut n = num;
        loop {
            if n == 0 {
                break
            };
            if n % 2 == 0 {
                n = n / 2;
            }
            else {
                n = n - 1;
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod test {
    use crate::number_of_steps_1342::Solution;

    #[test]
    fn test_solution1() {
        let target = Solution::number_of_steps(14);
        assert_eq!(target, 6)
    }

    #[test]
    fn test_solution2() {
        let target = Solution::number_of_steps(8);
        assert_eq!(target, 4)
    }
}