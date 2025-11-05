use std::sync::{Arc, atomic::{AtomicI32, Ordering}};
use std::thread;
use std::time::Instant;


fn main() {
    let inicio = Instant::now();
    let contador = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..500 {
        let c = Arc::clone(&contador);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                c.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    let duracao = inicio.elapsed();
    println!("Valor final: {}", contador.load(Ordering::Relaxed));
    println!("Tempo gasto: {:?}", duracao);
}
