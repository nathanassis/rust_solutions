use std::io;

fn read_input() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let input: Vec<&str> = input.split_whitespace().collect();

    input
        .into_iter()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

fn main() {
    let input = read_input();
    
    match input[0] {
        1 => println!("Total: R$ {:.2}", 4.0 * input[1] as f64),
        2 => println!("Total: R$ {:.2}", 4.5 * input[1] as f64),
        3 => println!("Total: R$ {:.2}", 5.0 * input[1] as f64),
        4 => println!("Total: R$ {:.2}", 2.0 * input[1] as f64),
        5 => println!("Total: R$ {:.2}", 1.5 * input[1] as f64),
        _ => unreachable!(),
    }
}
