// https://leetcode.com/problems/concatenation-of-array/

struct Solution;

impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let nums_clone = nums.clone();
        [nums, nums_clone].concat()
    }
}

fn main() {
    println!("{:?}", Solution::get_concatenation(vec![1, 2, 1]));
    println!("{:?}", Solution::get_concatenation(vec![1, 3, 2, 1]));
}
