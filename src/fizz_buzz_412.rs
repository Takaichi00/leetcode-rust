use std::ops::Add;
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = vec![];
        for num in 1..=n {

            let divisible3 = num % 3 == 0;
            let divisible5 = num % 5 == 0;

            let mut result_str = "".to_string();

            if divisible3 {
                result_str += "Fizz";
            }
            if divisible5 {
                result_str += "Buzz";
            }
            if result_str.is_empty() {
                result_str += &num.to_string();
            }
            result.push(result_str);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::fizz_buzz_412::Solution;

    #[test]
    fn _1を与えると1が返る() {
        let target = Solution::fizz_buzz(1);
        assert_eq!(target, vec!["1".to_string()])
    }

    #[test]
    fn _3を与えると1_2_fizzが返る() {
        let target = Solution::fizz_buzz(3);
        assert_eq!(target, vec!["1".to_string(), "2".to_string(), "Fizz".to_string()])
    }

    #[test]
    fn _5を与えると3はfizz_5はBuzzが返る() {
        let target = Solution::fizz_buzz(5);
        assert_eq!(target, vec!["1".to_string(), "2".to_string(), "Fizz".to_string(), "4".to_string(), "Buzz".to_string()])
    }
}