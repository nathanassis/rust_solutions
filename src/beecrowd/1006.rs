use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}

fn str_to_float(data: &String) -> f64 {
    data.trim().parse::<f64>().unwrap()
}

fn main() {
    let a: String = read_input();
    let b: String = read_input();
    let c: String = read_input();

    println!(
        "MEDIA = {:.1}",
        (2.0 * str_to_float(&a) + 3.0 * str_to_float(&b) + 5.0 * str_to_float(&c)) / 10.0
    );
}
