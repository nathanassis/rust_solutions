fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("");
    let dist = input.trim().parse::<i32>().unwrap();

    println!("{} minutos", dist * 2);
}