fn main() {
    let numbers = vec![1, 21, 3, 47, 5];

    let (soma, qtd) = numbers
        .iter()
        .filter(|&&x| x > 10)
        .fold((0, 0), |(soma, qtd), &x| (soma + x, qtd + 1));

    if qtd > 0 {
        println!("Média dos números maiores que 10: {:.2}", soma as f32 / qtd as f32);
    } else {
        println!("Nenhum número maior que 10 encontrado.");
    }
}
