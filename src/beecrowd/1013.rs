use std::cmp;
use std::io;

fn input_to_int() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let values: Vec<&str> = input.split_whitespace().collect();

    values
        .into_iter()
        .map(|val| val.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let values = input_to_int();

    println!(
        "{} eh o maior",
        cmp::max(cmp::max(values[0], values[1]), values[2])
    );
}
