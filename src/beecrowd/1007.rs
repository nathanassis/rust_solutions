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
    let a: i32 = str_to_int(&read_input());
    let b: i32 = str_to_int(&read_input());
    let c: i32 = str_to_int(&read_input());
    let d: i32 = str_to_int(&read_input());

    println!("DIFERENCA = {}", a * b - c * d);
}
