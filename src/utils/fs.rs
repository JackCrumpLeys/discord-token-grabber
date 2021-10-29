use std::fs;

pub fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn dir(path: &str) -> Vec<String> {
    let directory = fs::read_dir(path);

    if let Ok(directory) = directory {
        return directory
            .into_iter()
            .filter_map(|file| match file {
                Ok(file) => {
                    let path = file.path();

                    if path.is_dir() {
                        None
                    } else {
                        Some(path.display().to_string())
                    }
                }
                _ => None,
            })
            .collect::<Vec<String>>();
    }

    Vec::new()
}
