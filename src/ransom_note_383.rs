// https://leetcode.com/problems/ransom-note/
use derive_new::new;

#[derive(new)]
pub struct Solution {}

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {

        // magazine を1文字ずつの vec にする

        let char_vec: Vec<char> = magazine.chars().collect::<Vec<_>>();
        println!("{:#?}", char_vec);

        // n! 通りの文字を作成する (3文字だったら↓)
        // 0,1,2 / 0,2,1 / 1,0,2 / 1,2,0 / 2,0,1 / 2,1,0

        for n in char_vec.len() {
            let str = char_vec[0] + &char_vec[1] + &char_vec[2];
        }

        // ↑ の文字.contains ransam_note で答えが求まる



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

    #[test]
    fn test_solution4() {
        let target = Solution::can_construct("cba".to_string(), "abc".to_string());
        assert_eq!(target, true);
    }
}