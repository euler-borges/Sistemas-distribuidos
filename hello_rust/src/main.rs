use std::io;

fn main() {
    let mut input = String::new();
    println!("Qual o seu nome?");

    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim()); // Trim removes trailing newline
}
