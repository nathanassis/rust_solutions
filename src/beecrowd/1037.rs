use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    
    let number = input.trim().parse::<f32>().unwrap();

    if number >= 0.0 && number <= 25.0 { println!("Intervalo [0,25]"); }
    else if  number > 25.0 && number <= 50.0 { println!("Intervalo (25,50]"); }
    else if  number > 50.0 && number <= 75.0 { println!("Intervalo (50,75]"); }
    else if  number > 75.0 && number <= 100.0 { println!("Intervalo (75,100]"); }
    else { println!("Fora de intervalo"); }
}
