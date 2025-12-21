use mpi::collective::SystemOperation;
use mpi::traits::*;
use std::f64::consts::PI;
use std::time::Instant;

#[inline]
fn partial_sum(start: i64, end: i64, n: i64) -> f64 {
    let inv_n = 1.0 / n as f64;
    let mut sum = 0.0;

    for i in start..end {
        let x = (i as f64 + 0.5) * inv_n;
        sum += 4.0 / (1.0 + x * x);
    }

    sum
}

/// Cria um buffer "flat" para scatter: [start0, end0, start1, end1, ...]
fn make_ranges_flat(n: i64, p: i32) -> Vec<i64> {
    let p_i64 = p as i64;
    let chunk = n / p_i64;

    let mut flat = Vec::with_capacity((p as usize) * 2);
    for r in 0..p_i64 {
        let start = r * chunk;
        let end = if r == p_i64 - 1 { n } else { start + chunk };
        flat.push(start);
        flat.push(end);
    }
    flat
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();
    let size = world.size();

    // Ajuste conforme necessário
    let n: i64 = 100_000_000;

    // =========================
    // 1) Tempo serial (T1) no rank 0
    // =========================
    let mut t1_s: f64 = 0.0;

    if rank == 0 {
        let t0 = Instant::now();
        let s = partial_sum(0, n, n);
        let pi_serial = s / n as f64;
        t1_s = t0.elapsed().as_secs_f64();

        println!("=== Serial (rank 0) ===");
        println!("π serial: {:.15}", pi_serial);
        println!("T1:       {:.6} s", t1_s);
        println!();
    }

    // Broadcast do T1 (em segundos) para todo mundo (root = rank 0)
    world.process_at_rank(0).broadcast_into(&mut t1_s);

    // =========================
    // 2) Scatter: cada processo recebe [start,end] como 2 i64
    // =========================
    let mut my_range = [0_i64, 0_i64];

    if rank == 0 {
        let ranges_flat = make_ranges_flat(n, size);
        world
            .process_at_rank(0)
            .scatter_into_root(&ranges_flat[..], &mut my_range[..]);
    } else {
        world.process_at_rank(0).scatter_into(&mut my_range[..]);
    }

    let start = my_range[0];
    let end = my_range[1];

    // =========================
    // 3) Paralelo: mede Tp e faz Reduce (soma no root)
    // =========================
    world.barrier();
    let t_par = Instant::now();

    let local_sum = partial_sum(start, end, n);

    let mut global_sum = 0.0_f64;
    let op = SystemOperation::sum();

    if rank == 0 {
        world
            .process_at_rank(0)
            .reduce_into_root(&local_sum, &mut global_sum, op);
    } else {
        world.process_at_rank(0).reduce_into(&local_sum, op);
    }

    world.barrier();
    let tp_s = t_par.elapsed().as_secs_f64();

    // =========================
    // 4) Resultados + Speedup/Eficiência (rank 0)
    // =========================
    if rank == 0 {
        let pi_approx = global_sum / n as f64;

        let speedup = t1_s / tp_s;
        let efficiency = speedup / (size as f64);

        println!("=== Paralelo (Scatter + Reduce) ===");
        println!("Processos (P): {}", size);
        println!("π aproximado:  {:.15}", pi_approx);
        println!("π real:        {:.15}", PI);
        println!("Erro absoluto: {:.15}", (pi_approx - PI).abs());
        println!("Tp:            {:.6} s", tp_s);
        println!();
        println!("=== Métricas ===");
        println!("Speedup (T1/Tp): {:.4}", speedup);
        println!("Eficiência:      {:.4} ({:.2}%)", efficiency, efficiency * 100.0);
    }
}
