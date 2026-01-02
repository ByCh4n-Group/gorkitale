use tetra::Context;
use rand::Rng;
use crate::defs::Language;

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
    pub users: Vec<User>,
    pub current_user: Option<User>,
    pub panic_report: Vec<String>,
}

fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() {
        return vec!["".to_string()];
    }
    chars.chunks(max_chars)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
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

        Ok(Self {
            language: Language::English,
            users,
            current_user: None,
            panic_report: Vec::new(),
        })
    }

    pub fn generate_kernel_panic(&mut self) {
        let mut rng = rand::thread_rng();
        let reasons = [
            "Vibe check failed!",
            "Null pointer dereference in vibe_core.ko",
            "Stack overflow in chill_beats_module",
            "Out of memory: Kill process 'stress' (score 420)",
            "CPU 0: Machine Check Exception: Vibe Overload",
            "Fatal exception in interrupt handler: Bad Vibe",
            "Attempted to kill init! (exit code 0xdeadbeef)",
        ];
        let reason = reasons[rng.gen_range(0..reasons.len())];
        
        let mut lines = Vec::new();
        let max_chars = 75;

        let raw_lines = vec![
            format!("[    {:2}.{:06}] Kernel panic - not syncing: {}", rng.gen_range(10..99), rng.gen_range(0..999999), reason),
            format!("[    {:2}.{:06}] CPU: 0 PID: 420 Comm: vibecoded_game Tainted: G        W  O      6.9.420-vibecoded #1", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}] Hardware name: VibeCoded Virtual Machine/Standard PC (Q35 + ICH9, 2009), BIOS 1.0 12/31/2025", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}] Call Trace:", rng.gen_range(10..99), rng.gen_range(0..999999)),
            format!("[    {:2}.{:06}]  <TASK>", rng.gen_range(10..99), rng.gen_range(0..999999)),
        ];

        for raw in raw_lines {
            lines.extend(wrap_text(&raw, max_chars));
        }
        
        let symbols = ["dump_stack", "panic", "do_exit", "__handle_mm_fault", "do_group_exit", "get_signal", "arch_do_signal_or_restart", "exit_to_user_mode_prepare", "syscall_exit_to_user_mode", "do_syscall_64", "entry_SYSCALL_64_after_hwframe"];
        
        for sym in symbols {
            let offset = rng.gen_range(0x10..0xff);
            let size = rng.gen_range(0x100..0x500);
            let line = format!("[    {:2}.{:06}]  {}+0x{:x}/0x{:x}", rng.gen_range(10..99), rng.gen_range(0..999999), sym, offset, size);
            lines.extend(wrap_text(&line, max_chars));
        }
        
        let rip_line = format!("[    {:2}.{:06}] RIP: 0033:0x{:x}", rng.gen_range(10..99), rng.gen_range(0..999999), rng.r#gen::<u64>());
        lines.extend(wrap_text(&rip_line, max_chars));

        let task_end = format!("[    {:2}.{:06}]  </TASK>", rng.gen_range(10..99), rng.gen_range(0..999999));
        lines.extend(wrap_text(&task_end, max_chars));

        let end_panic = format!("[    {:2}.{:06}] ---[ end Kernel panic - not syncing: {} ]---", rng.gen_range(10..99), rng.gen_range(0..999999), reason);
        lines.extend(wrap_text(&end_panic, max_chars));

        lines.push("".to_string());
        lines.push("Press ENTER to reboot system...".to_string());
        
        self.panic_report = lines;
    }

    pub fn save_users(&mut self) {
        // Sync current_user back to users list
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
            content.push_str(&format!("{},{},{},{},{}\n", u.username, u.teblig_count, u.cihad_count, u.tekfir_count, u.current_stage));
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
}
