use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub id: u32,
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub id: u32,
    pub fileId: u32,
    pub schedule: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Status {
    Init,
    Disconnected,
    Connected,
    Running,
    Idle,
    Paused,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    pub files: Vec<File>,
    pub status: Status,
}

impl State {
    pub fn new() -> State {
        State {
            files: vec![],
            status: Status::Init,
        }
    }
}
