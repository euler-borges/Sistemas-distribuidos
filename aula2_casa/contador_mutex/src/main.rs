use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main () {
    let inicio = Instant::now();
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..500 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut val = counter_clone.lock().unwrap();
                *val += 1;
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }

    let duracao = inicio.elapsed();
    
    println!("Resultado: {}", *counter.lock().unwrap());
    println!("Tempo gasto: {:?}", duracao);
}