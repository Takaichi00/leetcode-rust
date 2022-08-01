// https://leetcode.com/problems/richest-customer-wealth/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {

        accounts.into_iter().map(|account| {
            account.iter().sum::<i32>()
        }).collect::<Vec<i32>>().into_iter().max().unwrap()

    }
}

#[cfg(test)]
mod test {
    use crate::richest_customer_wealth_1672::Solution;

    #[test]
    fn test_solution1() {
        let target = Solution::maximum_wealth(vec![vec![1,5],vec![7,3],vec![3,5]]);
        assert_eq!(target, 10)
    }

    #[test]
    fn test_solution2() {
        let target = Solution::maximum_wealth(vec![vec![2,8,7],vec![7,1,3],vec![1,9,5]]);
        assert_eq!(target, 17)
    }
}