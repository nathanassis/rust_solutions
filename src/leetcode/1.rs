// https://leetcode.com/problems/two-sum

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        impl Solution {
            pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
                let mut hm:std::collections::HashMap<i32,i32> = std::collections::HashMap::<i32,i32>::new();
                hm.insert(nums[0],0);
                for i in 1..nums.len() {
                    let c = nums[i];
                    let req = target - c;
                    if hm.contains_key(&req) {
                        let idx = match hm.get(&req) {
                            Some(x) => x,
                            None => &999
                        };
                        return vec![i as i32,*idx as i32];
                    }
                    hm.insert(c,i as i32);
                }
                return vec![];
            }
        }
    }
}

fn main() {
    let foo = Solution {};

    println!("{:?}", foo.two_sum(vec![2,7,11,15], 9));
}
