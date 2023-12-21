// Two Sum
// Solution 1
// 12 ms - 2.25 mb

struct Solution {}

impl Solution {
    pub fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index = 0;
        let mut target_index = usize::MAX;

        while index < nums.len() {
            target_index = nums[index+1..]
                .iter()
                .position(|&x| target - nums[index] == x)
                .unwrap_or(usize::MAX);

            if target_index != usize::MAX {
                break;
            }
            index += 1;
        }

        [index as i32, (1 + index + target_index) as i32].to_vec()
    }
}

fn main() {
    let foo = Solution {};

    println!("{:?}", foo.two_sum(vec![2,7,11,15], 9));
}
