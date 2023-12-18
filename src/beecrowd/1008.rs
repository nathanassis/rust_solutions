use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    input
}

fn main() {
    let func: i32 = read_input().trim().parse::<i32>().unwrap();
    let hrs: f64 = read_input().trim().parse::<f64>().unwrap();
    let sal: f64 = read_input().trim().parse::<f64>().unwrap();

    println!("NUMBER = {}\nSALARY = U$ {:.2}", func, hrs * sal);
}
