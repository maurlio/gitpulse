use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitHubEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub repo: Repo,
    pub payload: Payload,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub commits: Option<Vec<Commit>>,
    pub action: Option<String>,
    pub ref_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Commit {
    pub message: String,
}
