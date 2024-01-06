use std::io;

fn main() {
    let mut raw_value = String::new();
    io::stdin().read_line(&mut raw_value).expect("");
    let mut value = raw_value.trim().parse::<i32>().unwrap();

    let hora = value / 3600;
    value -= 3600 * hora;
    let minuto = value / 60;

    println!("{}:{}:{}", hora, minuto, value - 60 * minuto);
}
