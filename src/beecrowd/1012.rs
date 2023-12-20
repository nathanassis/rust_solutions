use std::io;

fn input_to_int() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let values: Vec<&str> = input.split_whitespace().collect();

    values
        .into_iter()
        .map(|val| val.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let values = input_to_int();

    println!(
        "TRIANGULO: {:.3}\nCIRCULO: {:.3}\nTRAPEZIO: {:.3}\nQUADRADO: {:.3}\nRETANGULO: {:.3}",
        { values[0] * values[2] / 2.0 },
        { values[2].powi(2) * PI },
        { (values[0] + values[1]) * values[2] / 2.0 },
        { values[1] * values[1] },
        { values[0] * values[1] }
    )
}
