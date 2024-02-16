use std::io;

fn read_input() -> Vec<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let values: Vec<&str> = input.split_whitespace().collect();

    values
        .into_iter()
        .map(|val| val.trim().parse::<f64>().unwrap())
        .collect()
}

fn calc_bhaskara(numbers: Vec<f64>) -> String {
    let delta: f64 = numbers[1].powi(2) - 4.0 * numbers[0] * numbers[2];
    if numbers[0] == 0.0 || delta < 0.0 {
        return "Impossivel calcular".to_string();
    }
    let r: [f64; 2] = [
        (-numbers[1] + delta.sqrt()) / (2.0 * numbers[0]),
        (-numbers[1] - delta.sqrt()) / (2.0 * numbers[0]),
    ];

    format!("R1 = {:.5}\nR2 = {:.5}", r[0], r[1])
}

fn main() {
    let input = read_input();
    println!("{}", calc_bhaskara(input));
}
