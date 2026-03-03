# GitPulse

GitPulse é uma ferramenta de linha de comando (CLI) minimalista desenvolvida em Rust para monitorar a atividade recente de usuários no GitHub em tempo real.

## Tecnologias Utilizadas

- Linguagem: [Rust](https://www.rust-lang.org/)
- Gerenciamento de Dependências: Cargo
- Comunicação HTTP: `ureq` 
- Serialização/JSON: `serde` & `serde_json`
- TLS/Criptografia: `rustls`

## Instalação e Compilação

Certifique-se de ter o [Rust e Cargo](https://rustup.rs/) instalados em sua máquina.

1. Clone o repositório:

```bash
git clone https://github.com/maurlio/gitpulse.git
cd gitpulse
```

2. Compile o projeto:
```bash
cargo build --release
```

O binário otimizado será gerado em `./target/release/gitpulse`.

## Como Usar

Para consultar a atividade de um usuário, execute o comando passando o nome de usuário como argumento:

```bash
$ gitpulse -- <username>
```

**Saida:**

```bash
$ gitpulse torvalds
Buscando atividades recentes de: @torvalds
Atividades de @torvalds:
- Interagiu com um Pull Request em microsoft/windows
- Fez push de 4 commit(s) em torvalds/linux
- Deu uma estrela (star) em nvidia/kernel-modules
```

## Testes

O projeto conta com testes de integração para garantir a estabilidade da interface CLI:

```bash
cargo test
```

## Estrutura do Projeto

```text
gitpulse/
├── src/
│   ├── main.rs          # Orquestração da aplicação
│   ├── cli.rs           # Interface de linha de comando
│   ├── http.rs          # Cliente HTTPS
│   ├── github.rs        # Modelagem de dados da API
│   └── json.rs          # Lógica de parsing e formatação
├── tests/               # Testes de integração
└── Cargo.toml           # Manifesto e dependências
```

## Melhorias Futuras

* Adicionar suporte a cores no terminal.
* Implementar cache local para reduzir requisições à API.
* Adicionar suporte a autenticação via Token para aumentar o Rate Limit.

## Licença

Este projeto é distribuído sob os termos da Licença MIT. Consulte o arquivo LICENSE para mais detalhes.
