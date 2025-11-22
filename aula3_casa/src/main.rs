use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use std::thread;
use std::time::{Duration, Instant}; 

struct BufferCompartilhado {
    buffer: VecDeque<i32>,
    capacidade: usize, //10
    itens_produzidos: usize,
    produtores_finalizados: usize,
    total_produtores: usize, // 3
}

struct EstadoCompartilhado {
    inner: Mutex<BufferCompartilhado>,
    condvar: Condvar,
}


fn produtor(id: usize, estado: Arc<EstadoCompartilhado>, quantidade: usize) {
    for i in 0..quantidade {
        let mut guard = estado.inner.lock().unwrap();
        while guard.buffer.len() == guard.capacidade {
            guard = estado.condvar.wait(guard).unwrap();
        }

        let item = (id as i32) * 1000 + i as i32; // algum valor identificável
        guard.buffer.push_back(item);
        guard.itens_produzidos += 1;
        println!("[Produtor {id}] Produziu item {item}. Buffer: {}", guard.buffer.len());

        estado.condvar.notify_all();
        drop(guard); // solta o lock antes de simular trabalho

        // Simula tempo de produção (opcional)
        thread::sleep(Duration::from_millis(10));
    }

    // sinaliza produtor finalizado
    let mut guard = estado.inner.lock().unwrap();
    guard.produtores_finalizados += 1;
    println!("[Produtor {id}] Finalizou. Produtores finalizados: {}", guard.produtores_finalizados);
    estado.condvar.notify_all();
}

fn consumidor(id: usize, estado: Arc<EstadoCompartilhado>) {
    loop {
        let item_opt;
        {
            let mut guard = estado.inner.lock().unwrap();

            loop {
                if let Some(item) = guard.buffer.pop_front() {
                    item_opt = Some(item);
                    println!("[Consumidor {id}] Consumiu item {item}. Buffer: {}", guard.buffer.len());
                    estado.condvar.notify_all();
                    break;
                } else {
                    // buffer vazio
                    if guard.produtores_finalizados == guard.total_produtores {
                        // nada mais virá
                        println!("[Consumidor {id}] Encerrando. Nenhum produtor ativo.");
                        return;
                    }
                    guard = estado.condvar.wait(guard).unwrap();
                }
            }
        }

        if let Some(_item) = item_opt {
            // Processar item fora do lock
            // Simula trabalho
            thread::sleep(Duration::from_millis(20));
        }
    }
}




fn main() {
    let runs = 20;
    let mut runtime = vec![];
    for run in 1..=runs {
        println!("--- Execução {} de {} ---", run, runs);
        let capacidade = 10;
        let total_produtores = 3;
        let total_consumidores = 2;
        let itens_por_produtor = 50;
        let mut handles = vec![];

        // estado compartilhado
        let estado = Arc::new(EstadoCompartilhado {
            inner: Mutex::new(BufferCompartilhado {
                buffer: VecDeque::with_capacity(capacidade),
                capacidade,
                itens_produzidos: 0,
                produtores_finalizados: 0,
                total_produtores,
            }),
            condvar: Condvar::new(),
        });

        let start = Instant::now();
        // threads
        for id in 0..total_produtores {
            let estado_clone = Arc::clone(&estado);
            let handle = thread::spawn(move || {
                produtor(id, estado_clone, itens_por_produtor);
            });
            handles.push(handle);
        }


        for id in 0..total_consumidores {
            let estado_clone = Arc::clone(&estado);
            let handle = thread::spawn(move || {
                consumidor(id, estado_clone);
            });
            handles.push(handle);
        }


        for handle in handles {
            handle.join().unwrap();
        }

        let duration = start.elapsed();

        println!("Todos os produtores e consumidores da run {} finalizaram em: {:?}", run, duration);
        runtime.push(duration);
    }

    let total_duration: Duration = runtime.iter().sum();
    let average_duration = total_duration / (runtime.len() as u32);
    println!("--- Estatísticas Finais ---");
    println!("Duração média das runs: {:?}", average_duration);
    println!("Duração de cada run:");
    for (i, dur) in runtime.iter().enumerate() {
        println!("Run {}: {:?}", i + 1, dur);
    }
}