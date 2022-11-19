pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, value) in nums.iter().enumerate(){
            match map.get(&(target-value)){
                Some(val) => {
                    return vec![index as i32, *val];
                }
                None => {
                    map.insert(*value, index as i32);
                }
            }
        }
        vec![]
    }
}

pub mod test {
    use super::*;
    pub fn test(){
        //assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9).sort() );
        let mut v = Solution::two_sum(vec![2, 7, 11, 15], 9);
        v.sort();
        assert_eq!(vec![0, 1], v);
        print!("tow sum test pass!");
    }
}