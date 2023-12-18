use std::io;

const PI: f64 = 3.14159;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}

fn str_to_float(data: &String) -> f64 {
    data.trim().parse::<f64>().unwrap()
}

fn main() {
    let r: String = read_input();

    println!("A={:.4}", PI * str_to_float(&r).powi(2));
}
