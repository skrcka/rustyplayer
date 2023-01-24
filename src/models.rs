use serde::{Deserialize, Serialize};
use tokio_cron_scheduler::Job;
use uuid::Uuid;

use crate::utils::load_media_files;
use crate::utils::load_schedules;
use crate::utils::write_media_files;
use crate::utils::write_schedules;
use crate::consts::MEDIA_PATH;


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

impl Schedule {
    pub fn new(id: u32, file_id: u32, schedule: String) -> Schedule {
        Schedule {
            id,
            file_id,
            schedule,
            activity: Activity::Inactive,
        }
    }
}

pub struct ActiveSchedule {
    pub schedule_id: u32,
    pub job_id: Uuid,
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

#[derive(Clone, Debug)]
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

    fn save_media(&self) {
        write_media_files(&self.files);
    }

    fn save_schedules(&self) {
        write_schedules(&self.schedules);
    }

    pub fn get_media(&self, id: u32) -> Option<&MediaFile> {
        self.files.iter().find(|f| f.id == id)
    }

    pub fn add_media(&mut self, name: String) {
        self.files.push(MediaFile::new(self.files.len() as u32, name));
        self.save_media();
    }

    pub fn remove_media(&mut self, id: u32) {
        self.files.retain(|f| f.id != id);
        self.save_media();
    }

    pub fn get_schedule(&self, id: u32) -> Option<&Schedule> {
        self.schedules.iter().find(|s| s.id == id)
    }

    pub fn get_mut_schedule(&mut self, id: u32) -> Option<&mut Schedule> {
        self.schedules.iter_mut().find(|s| s.id == id)
    }

    pub fn add_schedule(&mut self, file_id: u32, schedule: String) {
        self.schedules.push(Schedule::new(
            self.schedules.len() as u32,
            file_id,
            schedule,
        ));
        self.save_schedules();
    }

    pub fn remove_schedule(&mut self, id: u32) {
        self.schedules.retain(|s| s.id != id);
        self.save_schedules();
    }
}

impl MediaFile {
    pub fn new(id: u32, name: String) -> MediaFile {
        MediaFile {
            id: id,
            name: name.clone(),
            path: format!("{}{}", MEDIA_PATH, name),
        }
    }
}
