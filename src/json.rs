use crate::github::GitHubEvent;
use serde_json;

pub fn parse_events(json_str: &str) -> Result<Vec<GitHubEvent>, Box<dyn std::error::Error>> {
    let events: Vec<GitHubEvent> = serde_json::from_str(json_str)?;
    Ok(events)
}

pub fn format_event(event: &GitHubEvent) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            let count = event
                .payload
                .distinct_size
                .or(event.payload.size)
                .unwrap_or_else(|| event.payload.commits.as_ref().map_or(0, |c| c.len() as u32));
            
            let final_count = if count == 0 { 1 } else { count };
            format!("Fez push de {} commit(s) em {}", final_count, event.repo.name)
        }
        "IssuesEvent" => {
            let action = match event.payload.action.as_deref().unwrap_or("opened") {
                "opened" => "Abriu",
                "closed" => "Fechou",
                "reopened" => "Reabriu",
                other => other,
            };
            format!("{} uma issue em {}", action, event.repo.name)
        }
        "WatchEvent" => {
            format!("Deu uma estrela (star) em {}", event.repo.name)
        }
        "CreateEvent" => {
            let ref_type = match event.payload.ref_type.as_deref().unwrap_or("recurso") {
                "branch" => "branch",
                "repository" => "repositório",
                "tag" => "tag",
                other => other,
            };
            format!("Criou um novo {} em {}", ref_type, event.repo.name)
        }
        "PullRequestEvent" => {
            format!("Interagiu com um Pull Request em {}", event.repo.name)
        }
        _ => format!("Atividade de {} em {}", event.event_type, event.repo.name),
    }
}
