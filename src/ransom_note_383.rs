// https://leetcode.com/problems/ransom-note/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        magazine.contains(&ransom_note)
    }
}

#[cfg(test)]
mod test {
    use crate::ransom_note_383::Solution;

    #[test]
    fn test_solution1() {
        let target = Solution::can_construct("a".to_string(), "b".to_string());
        assert_eq!(target, false);
    }

    #[test]
    fn test_solution2() {
        let target = Solution::can_construct("aa".to_string(), "aab".to_string());
        assert_eq!(target, true);
    }

    #[test]
    fn test_solution3() {
        let target = Solution::can_construct("aab".to_string(), "baa".to_string());
        assert_eq!(target, true);
    }
}