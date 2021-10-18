use std::fs;

pub struct FS;

impl FS {
    pub fn read(path: &str) -> String {
        let file = fs::read(path);

        if let Ok(file) = file {
            return String::from_utf8(file).unwrap();
        }

        "".to_string()
    }

    pub fn dir(path: &str) -> Vec<String> {
        let result = fs::read_dir(path);

        if let Ok(result) = result {
            return result
                .into_iter()
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap().path())
                .filter(|r| !r.is_dir())
                .map(|r| r.display().to_string())
                .collect();
        }

        vec![]
    }
}
