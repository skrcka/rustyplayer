use std::fmt::format;
use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::Job;

use crate::utils::load_media_files;
use crate::utils::load_schedules;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MediaFile {
    pub id: u32,
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Activity {
    Active,
    Inactive,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Schedule {
    pub id: u32,
    pub file_id: u32,
    pub schedule: String,
    pub activity: Activity,
}

pub struct ActiveSchedule {
    pub id: u32,
    pub schedule_id: u32,
    pub job: Job,
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
    pub files: Vec<MediaFile>,
    pub schedules: Vec<Schedule>,
    pub status: Status,
}

impl State {
    pub fn new() -> State {
        State {
            files: vec![],
            schedules: vec![],
            status: Status::Init,
        }
    }

    pub fn load() -> State {
        State {
            files: load_media_files(),
            schedules: load_schedules(),
            status: Status::Idle,
        }
    }
}

impl MediaFile {
    pub fn new(id: u32, name: String) -> MediaFile {
        MediaFile {
            id: id,
            name: name.clone(),
            path: format!("media/{}", name),
        }
    }
}
