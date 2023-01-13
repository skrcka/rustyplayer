use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    pub name: String,
    pub path: String,
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
    pub pause: bool,
    pub status: Status,
}

impl State {
    pub fn new() -> State {
        State {
            files: vec![],
            pause: false,
            status: Status::Init,
        }
    }
}
