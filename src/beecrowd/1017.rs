use std::io;

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input.trim().parse::<i32>().unwrap()
}

fn main() {
    let tempo = read_input();
    let velocidade = read_input();

    println!("{:.3}", (tempo * velocidade) as f64 / 12.0);
}
