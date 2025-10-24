use std::collections::HashMap;
use std::io;

fn main() {
    let mut entrada = String::new();
    println!("Digite uma frase ou lista de palavras:");

    // Lê uma linha do teclado
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    // Cria um HashMap para armazenar as contagens
    let mut contador = HashMap::new();

    // Divide o texto em palavras (separadas por espaços)
    for palavra in entrada.split_whitespace() {
        // Converte para minúsculas (para evitar duplicatas tipo "Rust" e "rust")
        let palavra: String = palavra.to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect();

        // Incrementa o contador
        *contador.entry(palavra).or_insert(0) += 1;
    }

    // Exibe o resultado
    println!("\nFrequência das palavras:");
    for (palavra, contagem) in &contador {
        println!("{}: {}", palavra, contagem);
    }
}
