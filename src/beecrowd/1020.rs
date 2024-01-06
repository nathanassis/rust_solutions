use std::io;

fn main() {
    let mut raw_value = String::new();
    io::stdin().read_line(&mut raw_value).expect("");
    let mut value = raw_value.trim().parse::<i32>().unwrap();

    let ano = value / 365;
    value -= 365 * ano;
    let mes = value / 30;
    value -= 30 * mes;

    println!(
        "{} ano(s)\n{} mes(es)\n{} dia(s)",
        ano,
        mes,
        value - 30 * mes
    );
}
