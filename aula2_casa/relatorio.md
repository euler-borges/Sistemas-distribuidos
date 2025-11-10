# Relatório de implementações

Comparando-se as execuções dos 2 códigos, ambos com 500 threads contando até 1000, temos que, ao se executar os 2 códigos 3 vezes.

```bash
euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_atomic$ cargo run
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
Running `target/debug/contador_atomic`
Valor final: 500000
Tempo gasto: 29.244134ms

euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_atomic$ cargo run
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
Running `target/debug/contador_atomic`
Valor final: 500000
Tempo gasto: 18.855707ms

euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_atomic$ cargo run
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
Running `target/debug/contador_atomic`
Valor final: 500000
Tempo gasto: 18.328951ms
```

```bash
euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_mutex$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/contador_mutex`
Resultado: 500000
Tempo gasto: 90.260414ms

euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_mutex$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/contador_mutex`
Resultado: 500000
Tempo gasto: 71.770069ms

euler@EULER-PC:~/faculdade/SD/aula2_casa/contador_mutex$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/contador_mutex`
Resultado: 500000
Tempo gasto: 70.72937ms
```

Dessa forma, percebe-se que o uso do atomic é muito mais eficiente. Isso porque ele é feito para pequenas instruções de poucos passos, como é o caso do contador, possuindo assim performance maior.
