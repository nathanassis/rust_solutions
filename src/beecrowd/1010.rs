use std::io;

fn str_to_float(val: &str) -> f64 {
    val.trim().parse::<f64>().unwrap()
}

fn calcular(data: Vec<&str>) -> f64 {
    str_to_float(data[1]) * str_to_float(data[2])
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    let total = calcular(input.split_whitespace().collect());

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");

    println!(
        "VALOR A PAGAR: R$ {:.2}",
        total + calcular(input.split_whitespace().collect())
    );
}
