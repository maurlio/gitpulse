use std::io::Read;
use ureq;

pub struct HttpResponse {
    pub status_code: u16,
    pub body: String,
}

pub fn get_github_events(username: &str) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let url = format!("https://api.github.com/users/{}/events", username);

    let response = ureq::get(&url).set("User-Agent", "gitpulse-cli").call()?;
    let status_code = response.status();
    let mut body = String::new();

    response.into_reader().read_to_string(&mut body)?;

    Ok(HttpResponse { status_code, body })
}
