use std::io;

fn calc(value: &mut i32, div: i32) -> i32 {
    let num = *value / div;
    *value -= num * div;
    num
}

fn main() {
    let mut raw_value = String::new();
    io::stdin().read_line(&mut raw_value).expect("");
    let mut value = raw_value.trim().parse::<i32>().unwrap();

    let cem = calc(&mut value, 100);
    let cinq = calc(&mut value, 50);
    let vinte = calc(&mut value, 20);
    let dez = calc(&mut value, 10);
    let cinco = calc(&mut value, 5);
    let dois = calc(&mut value, 2);

    println!("{}{} nota(s) de R$ 100,00\n{} nota(s) de R$ 50,00\n{} nota(s) de R$ 20,00\n{} nota(s) de R$ 10,00\n{} nota(s) de R$ 5,00\n{} nota(s) de R$ 2,00\n{} nota(s) de R$ 1,00", raw_value, cem, cinq, vinte, dez, cinco, dois, value);
}
