use std::io;

pub fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
