use std::io;

fn read_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let values: Vec<&str> = input.split_whitespace().collect();

    values
        .into_iter()
        .map(|val| val.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let input = read_input();

    if input[1] > input[2] && input[3] > input[0] && input[2] + input[3] > input[0] + input[1] && input[2] > 0 && input[3] > 0 && input[0] % 2 == 0 {
        println!("Valores aceitos");
    } else {
        println!("Valores nao aceitos");
    }
}
