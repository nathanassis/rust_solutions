use std::io;

fn calc(value: &mut f64, div: i32) -> i32 {
    let num = *value as i32 / div;
    *value -= (num * div) as f64;
    num
}

fn main() {
    let mut raw_value = String::new();
    io::stdin().read_line(&mut raw_value).expect("");
    let mut value = raw_value.trim().parse::<f64>().unwrap();

    let cem = calc(&mut value, 100);
    let cinq = calc(&mut value, 50);
    let vinte = calc(&mut value, 20);
    let dez = calc(&mut value, 10);
    let cinco = calc(&mut value, 5);
    let dois = calc(&mut value, 2);
    let um = calc(&mut value, 1);

    value *= 100.0;
    let c_cinq = calc(&mut value, 50);
    let c_vcinco = calc(&mut value, 25);
    let c_dez = calc(&mut value, 10);
    let c_cinco = calc(&mut value, 5);

    println!(
        "NOTAS:\n{} nota(s) de R$ 100.00\n{} nota(s) de R$ 50.00\n{} nota(s) de R$ 20.00\n{} nota(s) de R$ 10.00\n{} nota(s) de R$ 5.00\n{} nota(s) de R$ 2.00\nMOEDAS:\n{} moeda(s) de R$ 1.00\n{} moeda(s) de R$ 0.50\n{} moeda(s) de R$ 0.25\n{} moeda(s) de R$ 0.10\n{} moeda(s) de R$ 0.05\n{} moeda(s) de R$ 0.01",
        cem, cinq, vinte, dez, cinco, dois, um, c_cinq, c_vcinco, c_dez, c_cinco, value as i32
    );
}
