// https://leetcode.com/problems/palindrome-number

struct Solution;

impl Solution {
    fn get_reverve(mut x: i32) -> i32 {
        let mut r = 0;
        while x > 0 {
            r = (r * 10) + (x % 10);
            x /= 10;
        }
        r
    }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        x == Self::get_reverve(x)
    }
}

fn main() {
    println!("{}", Solution::is_palindrome(121));
    println!("{}", Solution::is_palindrome(-121));
    println!("{}", Solution::is_palindrome(10));
}
