use crate::defs::Language;
use crate::global_db::GlobalSettings;
use tetra::Context;

#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
    pub teblig_count: u32,
    pub cihad_count: u32,
    pub tekfir_count: u32,
    pub current_stage: u32,
}

pub struct SystemState {
    pub language: Language,
    pub volume: f32,
    pub users: Vec<User>,
    pub current_user: Option<User>,
}

impl SystemState {
    pub fn new(_ctx: &mut Context) -> tetra::Result<Self> {
        let mut users = Vec::new();
        if let Ok(content) = std::fs::read_to_string("users.db") {
            for line in content.lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 5 {
                    users.push(User {
                        username: parts[0].to_string(),
                        teblig_count: parts[1].parse().unwrap_or(0),
                        cihad_count: parts[2].parse().unwrap_or(0),
                        tekfir_count: parts[3].parse().unwrap_or(0),
                        current_stage: parts[4].parse().unwrap_or(1),
                    });
                } else if parts.len() >= 4 {
                    // Backwards compatibility
                    users.push(User {
                        username: parts[0].to_string(),
                        teblig_count: parts[1].parse().unwrap_or(0),
                        cihad_count: parts[2].parse().unwrap_or(0),
                        tekfir_count: parts[3].parse().unwrap_or(0),
                        current_stage: 1,
                    });
                }
            }
        }

        let global_settings = GlobalSettings::new();
        let language = if global_settings.language == "tr" {
            Language::Turkish
        } else {
            Language::English
        };
        let volume = global_settings.volume as f32 / 100.0;

        Ok(Self {
            language,
            volume,
            users,
            current_user: None,
        })
    }

    pub fn save_users(&mut self) {
        // Sync current_user back to users list
        #[allow(clippy::collapsible_if)]
        if let Some(curr) = &self.current_user {
            if let Some(u) = self.users.iter_mut().find(|u| u.username == curr.username) {
                u.teblig_count = curr.teblig_count;
                u.cihad_count = curr.cihad_count;
                u.tekfir_count = curr.tekfir_count;
                u.current_stage = curr.current_stage;
            }
        }

        let mut content = String::new();
        for u in &self.users {
            content.push_str(&format!(
                "{},{},{},{},{}\n",
                u.username, u.teblig_count, u.cihad_count, u.tekfir_count, u.current_stage
            ));
        }
        std::fs::write("users.db", content).ok();
    }

    pub fn set_user_as_top(&mut self, index: usize) {
        if index < self.users.len() {
            let user = self.users.remove(index);
            self.users.insert(0, user);
            self.save_users();
        }
    }

    pub fn save_global_settings(&self) {
        let lang_str = match self.language {
            Language::English => "en",
            Language::Turkish => "tr",
        };
        let vol_int = (self.volume * 100.0) as u32;

        let settings = GlobalSettings {
            language: lang_str.to_string(),
            volume: vol_int,
        };
        settings.save();
    }
}
