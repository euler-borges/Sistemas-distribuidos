use std::sync::mpsc;
use std::thread;

fn main() {
    // Cria um canal (tx = transmissor, rx = receptor)
    let (tx, rx) = mpsc::channel();

    // Thread produtora
    let produtor = thread::spawn(move || {
        for i in 1..=100 {
            println!("Produzindo: {}", i);
            tx.send(i).unwrap(); // envia o número para o consumidor
        }
        println!("Produção finalizada!");
    });

    // Thread consumidora
    let consumidor = thread::spawn(move || {
        for recebido in rx {
            println!("Consumindo: {}", recebido);
        }
        println!("Consumo finalizado!");
    });

    // Aguarda as threads terminarem
    produtor.join().unwrap();
    consumidor.join().unwrap();
}
