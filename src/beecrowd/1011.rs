use std::io;

const PI: f64 = 3.14159;

fn input_to_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input.trim().parse::<f64>().unwrap()
}

fn main() {
    let raio = input_to_float();

    println!("VOLUME = {:.3}", 4.0 * PI * raio.powi(3) / 3.0);
}
