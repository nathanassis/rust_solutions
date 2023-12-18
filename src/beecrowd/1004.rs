use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}

fn str_to_int(data: &String) -> i32 {
    data.trim().parse::<i32>().unwrap()
}

fn main() {
    let a: String = read_input();
    let b: String = read_input();

    println!("PROD = {}", str_to_int(&a) * str_to_int(&b));
}
