fn adiciona_sufixo(mut s: String, sufixo: &str) -> String {
    s.push_str(sufixo); // modifica a string no local (sem cópia)
    s
}

fn main() {
    let string1 = String::from("Rust");
    let string1 = adiciona_sufixo(string1, "_lang");

    println!("{}", string1);
    
}
