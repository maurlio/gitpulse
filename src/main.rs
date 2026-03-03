mod cli;
mod github;
mod http;
mod json;

use cli::Config;
use std::process;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!(
        "Buscando atividades recentes de: @{}",
        config.username
    );

    match http::get_github_events(&config.username) {
        Ok(response) => {
            match json::parse_events(&response.body) {
                Ok(events) => {
                    if events.is_empty() {
                        println!("Nenhuma atividade recente encontrada para este usuário.");
                    } else {
                        println!("Atividades de @{}:", config.username);
                        for event in events.iter().take(10) {
                            println!("- {}", json::format_event(event));
                        }
                    }
                }
                Err(e) => eprintln!("Erro ao processar dados (JSON): {}", e),
            }
        }
        Err(e) => {
            if let Some(ureq::Error::Status(code, _)) = e.downcast_ref::<ureq::Error>() {
                if *code == 404 {
                    eprintln!("Erro: Usuário '@{}' não encontrado.", config.username);
                } else {
                    eprintln!("Erro na API do GitHub: Status {}", code);
                }
            } else {
                eprintln!("Erro de conexão: {}", e);
            }
            process::exit(1);
        }
    }
}