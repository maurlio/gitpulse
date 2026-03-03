mod cli;
mod github;
mod http;
mod json;

use cli::Config;
use std::process;
use ureq::Response;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!(
        "Conectando ao GitHub para obter eventos do usuário: @{}",
        config.username
    );

    match http::get_github_events(&config.username) {
        Ok(response) => {
            println!("Sucesso! Eventos capturados.");
        }
        Err(e) => {
            if let Some(ureq::Error::Status(code, _)) = e.downcast_ref::<ureq::Error>() {
                if *code == 404 {
                    eprintln!("Erro: Usuário '@{}' não encontrado no GitHub.", config.username);
                } else {
                    eprintln!("Erro na API do GitHub: Status {}", code);
                }
            } else {
                eprintln!("Erro ao conectar ao GitHub: {}", e);
            }
            std::process::exit(1);
        }
    }
}
