// :: para tipagem

extern crate rand; // estamos avisando o Rust que estamos utilizando libs externas.

use std::io; // Biblioteca de entrada de input e output, IO entendeu??
use rand::Rng;  // Biblioteca para gerar um número aleatorio
use std::cmp::Ordering; // Biblioteca de comparação

fn main() { // a Função principal, ponto de partida do programa
    println!("Adivinhe o número");

    // Cria um número aleatorio de 1 a 100
    let aleNum = rand::thread_rng().gen_range(1, 101);

    loop { // um loop infinito, e insaciável gostei dessa expressão!!
        println!("Digite o seu palpite: ");

        // ::new() para criar uma string vazia
        let mut palpite = String::new(); // declarar o mut para tornar uma variável mutavel

        // Resumo do livro -> 
        // Para resumir, a linha let mut palpite = String::new(); 
        // criou uma variável mutável que está atualmente vinculada a uma nova instância vazia de uma String. Ufa!

        io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler entrada");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // Tratativa de erros, caso não seja um numero ele continua e recomeça.

        println!("Você disse: {}", palpite); // ! no final do print para poder usar as variáveis

        match palpite.cmp(&aleNum) { // Função para comparação
            Ordering::Less => println!("baixo"),
            Ordering::Greater => println!("alto"),
            Ordering::Equal => { // caso esteja igual, o programa fechara
                println!("acertou!!!");
                break;
            }
        }
    }
    
}
