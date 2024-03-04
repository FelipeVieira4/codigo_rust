use std::f32::consts::PI;
use std::io;

fn calcular_raio(raio: f32) {
    println!("O raio do círculo que é {} é: {}", raio, PI * raio.powf(2.0));
}

fn main() {
    let io_input = io::stdin();
    let mut input = String::new();
    let valor_raio: f32;
    
    println!("Digite o valor do raio:");
    
    io_input.read_line(&mut input).expect("Erro ao ler a linha");
    valor_raio = input.trim().parse().expect("Insira um valor válido");

    calcular_raio(valor_raio);
}