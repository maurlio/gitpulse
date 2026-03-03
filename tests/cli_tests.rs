use std::process::Command;

#[test]
fn test_no_args_provided() {
    // Tenta executar o binário sem argumentos
    let output = Command::new("cargo")
        .args(&["run", "--"])
        .output()
        .expect("Falha ao executar o comando");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verifica se a mensagem de erro que definimos no cli.rs aparece
    assert!(stderr.contains("Erro: Nome de usuário não fornecido."));
}

#[test]
fn test_empty_username() {
    // Tenta executar com um nome de usuário vazio
    let output = Command::new("cargo")
        .args(&["run", "--", " "])
        .output()
        .expect("Falha ao executar o comando");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stderr.contains("Erro: Nome de usuário não pode ser vazio."));
}

#[test]
fn test_user_not_found() {
    // Busca por um usuário que provavelmente não existe (nome aleatório longo)
    let output = Command::new("cargo")
        .args(&["run", "--", "usuario-que-nao-existe-1234567890"])
        .output()
        .expect("Falha ao executar");

    let stderr = String::from_utf8_lossy(&output.stderr);

    // O programa deve exibir nossa mensagem customizada de erro 404
    assert!(stderr.contains("não encontrado"));
}
