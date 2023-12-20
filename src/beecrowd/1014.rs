use std::io;

fn main() {
    let mut distancia = String::new();
    io::stdin().read_line(&mut distancia).expect("");
    let distancia = distancia.trim().parse::<i32>().unwrap();

    let mut gasto = String::new();
    io::stdin().read_line(&mut gasto).expect("");
    let gasto = gasto.trim().parse::<f64>().unwrap();

    println!("{:.3} km/l", distancia as f64 / gasto);
}
