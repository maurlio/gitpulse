mod cli;
mod http;
mod json;
mod github;

use std::process;
use cli::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Buscando atividades do usuário: @{}", config.username);
}
