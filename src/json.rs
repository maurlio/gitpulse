use crate::github::GitHubEvent;
use serde_json;

pub fn parse_events(json_str: &str) -> Result<Vec<GitHubEvent>, Box<dyn std::error::Error>> {
    let events: Vec<GitHubEvent> = serde_json::from_str(json_str)?;
    Ok(events)
}

pub fn format_event(event: &GitHubEvent) -> String {
    match event.event_type.as_str() {
        "PushEvent" => {
            let count = event.payload.commits.as_ref().map_or(0, |c| c.len());
            format!("Pushed {} commit(s) to {}", count, event.repo.name)
        }
        "IssuesEvent" => {
            let action = event.payload.action.as_deref().unwrap_or("opened");
            format!("{} an issue in {}", action, event.repo.name)
        }
        "WatchEvent" => {
            format!("Starred {}", event.repo.name)
        }
        "CreateEvent" => {
            let ref_type = event.payload.ref_type.as_deref().unwrap_or("resource");
            format!("Created a new {} in {}", ref_type, event.repo.name)
        }
        _ => format!("{} in {}", event.event_type, event.repo.name),
    }
}
