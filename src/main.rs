/* Atividade: Um jogo de adivinhar um número gerado automaticamente pelo código.
O programa deve informar e permitir que o jogador insira números;
O programa deve gerar um número aleatório entre 1 a 1000;
O programa deve informar ao jogador se o número inserido por ele está abaixo, acima ou é igual o número aleatório;
O programa deve sair automaticamente quando o jogador adivinhar o número ou quando ele inserir qualquer valor que não seja um número. */

use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Adivinha um número");
    let secret = rand::thread_rng().gen_range(1, 1001);

    loop {
        println!("Por favor, Digite seu palpite de um número");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha de comando.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("O número inserido é: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("Você ganhou!");
                break;
            },
            Ordering::Less => println!("Muito abaixo"),
            Ordering::Greater => println!("Muito acima"),
        }
    }
}
