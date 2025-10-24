use std::env;

fn calcular(a: f64, b: f64, op: &str) -> Result<f64, String> {
    match op {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(String::from("Erro: divisão por zero"))
            } else {
                Ok(a / b)
            }
        }
        _ => Err(format!("Operação inválida: {}", op)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Uso: cargo run <num1> <operação> <num2>");
        return;
    }

    let a: f64 = args[1].parse().unwrap_or_else(|_| {
        eprintln!("Erro: '{}' não é um número válido", args[1]);
        std::process::exit(1);
    });

    let b: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("Erro: '{}' não é um número válido", args[3]);
        std::process::exit(1);
    });

    let op = &args[2];

    match calcular(a, b, op) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(erro) => eprintln!("{}", erro),
    }
}
