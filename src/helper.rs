use std::collections::HashMap;

#[derive(Debug)]
pub struct ParsedHttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

pub fn parse_curl_request(curl_command: &str) -> Result<ParsedHttpRequest, &'static str> {
    let mut method = "GET".to_string();
    let mut url = String::new();
    let mut headers = HashMap::new();
    let mut body = None;

    // Split the cURL command into parts (naively by whitespace for now)
    let parts: Vec<&str> = curl_command.split_whitespace().collect();
    
    let mut i = 0;
    while i < parts.len() {
        match parts[i] {
            "-X" => {
                // Extract the HTTP method
                if i + 1 < parts.len() {
                    method = parts[i + 1].to_string();
                    i += 1;
                }
            },
            "-H" => {
                // Extract headers
                if i + 1 < parts.len() {
                    let header_part = parts[i + 1];
                    if let Some((key, value)) = header_part.split_once(":") {
                        headers.insert(key.trim().to_string(), value.trim().to_string());
                    }
                    i += 1;
                }
            },
            "-d" | "--data" => {
                // Extract the body data
                if i + 1 < parts.len() {
                    body = Some(parts[i + 1].to_string());
                    i += 1;
                }
            },
            _ if parts[i].starts_with("http") => {
                // Assume the URL
                url = parts[i].to_string();
            },
            _ => {}
        }
        i += 1;
    }

    if url.is_empty() {
        return Err("URL not found in the cURL command");
    }

    Ok(ParsedHttpRequest {
        method,
        url,
        headers,
        body,
    })
}


pub fn extract_body_from_curl(curl_command: &str) -> Option<String> {
    // Split the cURL command into parts, handling quoted parts correctly
    let parts: Vec<String> = shell_words::split(curl_command).unwrap_or_default();

    // Iterate over parts to find the `-d` or `--data` flag and extract the body
    let mut i = 0;
    while i < parts.len() {
        match parts[i].as_str() {
            "-d" | "--data" => {
                // Extract the body data that comes after `-d` or `--data`
                if i + 1 < parts.len() {
                    return Some(parts[i + 1].clone());
                }
            },
            _ => {}
        }
        i += 1;
    }

    // If no body is found, return None
    None
}