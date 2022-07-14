use derive_new::new;

#[derive(new)]
// https://leetcode.com/problems/running-sum-of-1d-array/submissions/
pub struct Solution {}

impl Solution {
    pub fn running_sum(&self, nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();

        for n in 0..nums.len() {
            if n == 0 {
                result.push(*nums.get(n).unwrap());
            } else {
                let mut tmp = 0;
                for j in 0..=n {
                    tmp += *nums.get(j).unwrap();
                }

                result.push(tmp);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::running_sum_of_array::Solution;

    #[test]
    fn test_solution() {
        let target = Solution::new();
        target.running_sum(vec![1, 2, 3, 4]);
    }
}
