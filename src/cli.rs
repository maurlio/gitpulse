use std::env;

pub struct Config {
    pub username: String,
}

impl Config {
    pub fn build() -> Result<Config, &'static str> {
        let mut args = env::args();

        args.next();

        let username = match args.next() {
            Some(arg) => arg,
            None => return Err("Erro: Nome de usuário não fornecido."),
        };

        if username.trim().is_empty() {
            return Err("Erro: Nome de usuário não pode ser vazio.");
        }

        Ok(Config { username })
    }
}
