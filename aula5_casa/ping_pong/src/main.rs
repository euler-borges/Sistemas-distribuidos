use mpi::traits::*;
use std::time::{Duration, Instant};

/// Executa ping-pong entre exatamente 2 processos (rank 0 e 1),
/// incrementando um contador até `max_msgs`.
///
/// Retorna `Some(Duration)` apenas no rank 0, e `None` no rank 1.
fn ping_pong_benchmark<C: Communicator>(world: &C, rank: i32, size: i32, max_msgs: i32) -> Option<Duration> {
    if size != 2 {
        if rank == 0 {
            eprintln!("Este programa requer exatamente 2 processos");
        }
        return None;
    }

    let partner_rank = 1 - rank; // 0 ↔ 1
    let mut ping_pong_count: i32 = 0;

    // Sincroniza para iniciar junto
    world.barrier();

    let start = if rank == 0 { Some(Instant::now()) } else { None };

    while ping_pong_count < max_msgs {
        if rank == (ping_pong_count % 2) {
            ping_pong_count += 1;
            world.process_at_rank(partner_rank).send(&ping_pong_count);
        } else {
            world
                .process_at_rank(partner_rank)
                .receive_into(&mut ping_pong_count);
        }
    }

    // Sincroniza para terminar junto
    world.barrier();

    if rank == 0 {
        Some(start.unwrap().elapsed())
    } else {
        None
    }
}

/// Roda o benchmark com diferentes quantidades de mensagens e imprime resultados (no rank 0).
fn run_benchmarks<C: Communicator>(world: &C, rank: i32, size: i32) {
    let tests = [1_000i32, 10_000i32, 100_000i32];

    for &max_msgs in &tests {
        let duration = ping_pong_benchmark(world, rank, size, max_msgs);

        if let Some(d) = duration {
            // 1 incremento == 1 mensagem enviada
            let total_msgs_u32 = max_msgs.max(1) as u32;

            // Cada ida-e-volta ~ 2 mensagens; evita zero
            let round_trips_u32 = (max_msgs / 2).max(1) as u32;

            let avg_per_msg = d / total_msgs_u32;
            let avg_per_round_trip = d / round_trips_u32;

            println!("=== Benchmark ping-pong ===");
            println!("Mensagens (total): {}", max_msgs);
            println!("Tempo total: {:?}", d);
            println!("Latência média por mensagem: {:?}", avg_per_msg);
            println!("Latência média por ida-e-volta (~2 msgs): {:?}", avg_per_round_trip);
            println!();
        }
    }
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let rank = world.rank();
    let size = world.size();

    run_benchmarks(&world, rank, size);
}
