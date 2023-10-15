use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    server: Server,
    pub problems: Vec<Problem>,
    pub languages: Vec<Language>,
}

#[derive(Clone, Deserialize, Serialize)]
struct Server {
    bind_address: Option<String>,
    bind_port: Option<i32>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Problem {
    pub id: usize,
    name: String,
    pub cases: Vec<Case>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Case {
    pub input_file: String,
    pub answer_file: String,
    pub time_limit: u32,
    pub memory_limit: u32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Language {
    pub name: String,
    pub file_name: String,
    pub command: Vec<String>,
}

impl Language {
    pub fn replace(&mut self, before: &str, after: &str) {
        self.command.iter_mut().for_each(|cmd| {
            if *cmd == before {
                *cmd = after.to_string();
            }
        });
    }
}
