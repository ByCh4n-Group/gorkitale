use std::fs;

pub struct GlobalSettings {
    pub language: String,
    pub volume: u32,
}

impl GlobalSettings {
    pub fn new() -> Self {
        Self::load()
    }

    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string("global.db") {
            let parts: Vec<&str> = content.trim().split(',').collect();
            if parts.len() >= 2 {
                let language = parts[0].to_string();
                let volume = parts[1].parse().unwrap_or(100);
                return Self { language, volume };
            }
        }
        // Defaults
        Self {
            language: "en".to_string(),
            volume: 100,
        }
    }

    pub fn save(&self) {
        let content = format!("{},{}", self.language, self.volume);
        let _ = fs::write("global.db", content);
    }
}
