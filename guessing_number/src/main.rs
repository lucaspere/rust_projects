use guessing_game::Config;
use rand::Rng;
use std::process;

fn main() {
    println!("Adivinha um número");
    let secret = rand::thread_rng().gen_range(1, 1001);

    if let Err(err) = Config::new(secret).run() {
        eprintln!("{}", err);

        process::exit(1)
    }
}
