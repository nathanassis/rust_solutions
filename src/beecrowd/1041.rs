use std::io;

fn read_input() -> Vec<f32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let input: Vec<&str> = input.split_whitespace().collect();

    input
        .into_iter()
        .map(|x| x.trim().parse::<f32>().unwrap())
        .collect()
}

fn main() {
    let points = read_input();
    
    if points[0] == 0.0 && points[1] == 0.0 {
        println!("Origem");
    } else if points[0] == 0.0 {
        println!("Eixo X");
    } else if points[1] == 0.0 {
        println!("Eixo Y");
    } else if points[0] > 0.0 && points[1] > 0.0 {
        println!("Q1");
    } else if points[0] < 0.0 && points[1] > 0.0 {
        println!("Q2");
    } else if points[0] < 0.0 && points[1] < 0.0 {
        println!("Q3");
    } else {
        println!("Q4")
    }
}
