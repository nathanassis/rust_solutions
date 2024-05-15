// https://leetcode.com/problems/roman-to-integer

struct Solution;

impl Solution {
    fn convert_char_to_int(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => unreachable!(),
        }
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let mut last_num = 0;
        for c in s.chars().rev() {
            let num = Self::convert_char_to_int(c);
            if last_num > num {
                res -= num;
            } else {
                res += num;
            }
            last_num = num;
        }

        res
    }
}

fn main() {
    println!("{}", Solution::roman_to_int(String::from("III")));
    println!("{}", Solution::roman_to_int(String::from("LVIII")));
    println!("{}", Solution::roman_to_int(String::from("MCMXCIV")));
}
