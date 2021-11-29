use std::io;

fn main() {
    println!("Enter first number: ");
    let mut var1 = String::new(); // Mútavel pois pegaremos o valor do terminal e iremos manipular
    io::stdin().read_line(&mut var1).expect("Program only proccesses numbers"); // Usamos read_line para pegar o que foi digitado e armanezar na variável mútavel
    
    println!("Enter second number: ");
    let mut var2 = String::new();
    io::stdin().read_line(&mut var2).expect("Program only proccesses numbers");

    let a: i32 = var1.trim().parse().ok().expect("Program only proccesses numbers"); // Convertemos o valor recebido em string para inteiro
    let b: i32 = var2.trim().parse().ok().expect("Program only proccesses numbers");

    let x = a + b; // Realizamos a soma
    println!("X = {}", x); // Exibimos a soma no final de tudo
}
