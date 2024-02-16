// https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/
// NOT SOLVED!

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for item in arr {
            let count = hash_map.entry(item).or_insert(0);
            *count += 1;
        }

        let mut min_value = hash_map
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(x, _y)| x).unwrap();
        for _ in 0..k {
            let (key, value): (&i32, &i32) = hash_map.get_key_value(min_value).unwrap();
            match *value {
                1 => {
                    hash_map.remove(&key.clone());
                    min_value = hash_map.iter().min_by(|a, b| a.1.cmp(&b.1)).map(|(x, _y)| x).unwrap();
                },
                _ => {
                    hash_map.clone().entry(*key).and_modify(|v| { *v -= 1 });
                }
            }
        }

        hash_map.len() as i32
    }
}

fn main() {
    // println!("{}", Solution::find_least_num_of_unique_ints(vec![5, 5, 4], 1)); // 1
    // println!("{}",Solution::find_least_num_of_unique_ints(vec![4, 3, 1, 1, 3, 3, 2], 3)); // 2
    
    // it fails in the next example
    println!("{}",Solution::find_least_num_of_unique_ints(vec![2, 1, 1, 3, 3, 3], 3)); // expect: 1  receive: 2
}
