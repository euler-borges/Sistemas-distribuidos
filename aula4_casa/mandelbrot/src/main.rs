use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

/// Resolução da imagem
const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

/// Número máximo de iterações do Mandelbrot
const MAX_ITER: u32 = 1000;

/// Representa um número complexo simples (re + im·i)
#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    /// Cria um novo número complexo
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Norma ao quadrado: |z|² = re² + im²
    fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

/// Implementa multiplicação de complexos: (a+bi)·(c+di)
impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// Implementa soma de complexos: (a+bi) + (c+di)
impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

/// Converte coordenadas de pixel (x, y) para um ponto no plano complexo.
///
/// A janela padrão do Mandelbrot é algo próximo de:
///   x ∈ [-2.5, 1.0]
///   y ∈ [-1.0, 1.0]
fn pixel_to_complex(x: usize, y: usize, width: usize, height: usize) -> Complex {
    let x_min = -2.5;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;

    let re = x_min + (x as f64 / width as f64) * (x_max - x_min);
    let im = y_min + (y as f64 / height as f64) * (y_max - y_min);

    Complex::new(re, im)
}

/// Calcula o número de iterações até escapar (ou MAX_ITER se permanecer dentro do conjunto).
fn mandelbrot(c: Complex) -> u32 {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..MAX_ITER {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }

    MAX_ITER
}

/// Renderização sequencial do conjunto de Mandelbrot.
///
/// Retorna um vetor linear (flatten) de tamanho width * height.
/// O pixel (x, y) fica em index = y * width + x.
fn render_mandelbrot_seq(width: usize, height: usize) -> Vec<u32> {
    let mut data = vec![0u32; width * height];

    for y in 0..height {
        for x in 0..width {
            let c = pixel_to_complex(x, y, width, height);
            let iter = mandelbrot(c);
            let idx = y * width + x;
            data[idx] = iter;
        }
    }

    data
}

/// Renderização paralela usando Rayon.
///
/// Também retorna um vetor linear de tamanho width * height.
fn render_mandelbrot_par(width: usize, height: usize) -> Vec<u32> {
    // Estratégia: paralelizar nas linhas (y) e "achatar" as linhas em um único vetor.
    (0..height)
        .into_par_iter()
        .flat_map_iter(|y| {
            (0..width).map(move |x| {
                let c = pixel_to_complex(x, y, width, height);
                mandelbrot(c)
            })
        })
        .collect()
}

/// Salva os dados como imagem em formato PGM (P2, ASCII).
///
/// Cada valor de iteração é normalizado para a faixa [0, 255].
fn save_as_pgm(
    filename: &str,
    data: &[u32],
    width: usize,
    height: usize,
) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);

    // Cabeçalho PGM
    writeln!(writer, "P2")?; // Formato ASCII
    writeln!(writer, "{} {}", width, height)?;
    writeln!(writer, "255")?; // Valor máximo (8 bits)

    // Corpo da imagem
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let iter = data[idx];

            // Normaliza: 0..MAX_ITER -> 0..255
            let normalized =
                ((iter as f64 / MAX_ITER as f64) * 255.0).clamp(0.0, 255.0) as u32;

            write!(writer, "{} ", normalized)?;
        }
        writeln!(writer)?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Renderizando Mandelbrot {}x{} (sequencial vs paralelo)...",
        WIDTH, HEIGHT
    );

    // ==========================
    // 1) Versão sequencial
    // ==========================
    let start_seq = Instant::now();
    let img_seq = render_mandelbrot_seq(WIDTH, HEIGHT);
    let duration_seq = start_seq.elapsed();
    let t_seq = duration_seq.as_secs_f64();

    println!("Tempo SEQUENCIAL: {:.3} s", t_seq);

    // ==========================
    // 2) Versão paralela (1, 2, 4, 8 threads)
    // ==========================
    let thread_counts = [1_usize, 2, 4, 8];

    println!("\nResultados paralelos (comparados com o sequencial):");
    println!("Threads;Tempo(s);Speedup;Eficiência");

    // Guarda uma imagem paralela para salvar (pode ser a com maior número de threads)
    let mut img_to_save: Option<Vec<u32>> = None;

    for &n_threads in &thread_counts {
        // Cria um pool de threads Rayon com n_threads
        let pool = ThreadPoolBuilder::new()
            .num_threads(n_threads)
            .build()
            .expect("Não foi possível criar o thread pool do Rayon");

        let start_par = Instant::now();

        // Executa a renderização paralela dentro deste pool
        let img_par = pool.install(|| render_mandelbrot_par(WIDTH, HEIGHT));

        let duration_par = start_par.elapsed();
        let t_par = duration_par.as_secs_f64();

        // Calcula speedup e eficiência
        let speedup = t_seq / t_par;
        let efficiency = speedup / (n_threads as f64);

        println!(
            "{};{:.3};{:.3};{:.3}",
            n_threads, t_par, speedup, efficiency
        );

        // Guarda a imagem paralela de maior número de threads para salvar
        if n_threads == *thread_counts.last().unwrap() {
            img_to_save = Some(img_par);
        }
    }

    // ==========================
    // 3) Salvamento da imagem
    // ==========================
    // Aqui você pode escolher salvar a imagem sequencial ou paralela.
    // A imagem é a mesma, então tanto faz para visualização.
    let final_image = img_to_save.unwrap_or(img_seq);
    let filename = "mandelbrot_1000x1000.pgm";

    save_as_pgm(filename, &final_image, WIDTH, HEIGHT)?;
    println!("\nImagem salva como '{}'", filename);

    Ok(())
}
