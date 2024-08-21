#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub window_title: String,
}

impl Config {
    pub fn read(path: &str) -> Self {
        let contents = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&contents).unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: String::from("Default title"),
        }
    }
}
