use std::io;

fn read_input() -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let input: Vec<&str> = input.split_whitespace().collect();

    input
        .into_iter()
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect()
}

fn main() {
    let x = read_input();
    let y = read_input();

    println!(
        "{:.4}",
        ((y[0] - x[0]).powi(2) + (y[1] - x[1]).powi(2)).sqrt()
    );
}
