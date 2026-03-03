use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct GitHubEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: Repo,
    pub payload: Payload,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Payload {
    pub commits: Option<Vec<Commit>>,
    pub action: Option<String>,
    pub ref_type: Option<String>,
    pub size: Option<u32>, // Adicionado para capturar o número de commits de forma alternativa
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub message: String,
}