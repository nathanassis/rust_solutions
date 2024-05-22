// https://leetcode.com/problems/build-array-from-permutation/

struct Solution;

impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[i] = nums[nums[i] as usize];
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::build_array(vec![0, 2, 1, 5, 3, 4]));
    println!("{:?}", Solution::build_array(vec![5, 0, 1, 2, 3, 4]));
}
