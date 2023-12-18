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
    read_input();
    let salario: f64 = str_to_float(&read_input());
    let vendas: f64 = str_to_float(&read_input());

    println!("TOTAL = R$ {:.2}", salario + vendas * 0.15);
}
