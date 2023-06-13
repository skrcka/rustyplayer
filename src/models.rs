use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

use crate::utils::load_media_files;
use crate::utils::load_schedules;
use crate::utils::write_media_files;
use crate::utils::write_schedules;

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Status {
    Init,
    Disconnected,
    Connected,
    Running,
    Idle,
    Paused,
}

#[derive(Debug)]
pub struct IdGenerator {
    id: AtomicUsize,
}

impl IdGenerator {
    pub fn new(start: u32) -> IdGenerator {
        IdGenerator {
            id: AtomicUsize::new(start as usize),
        }
    }

    pub fn next(&self) -> u32 {
        self.id.fetch_add(1, Ordering::SeqCst);
        self.id.load(Ordering::SeqCst) as u32
    }
}

#[derive(Debug)]
pub struct State {
    pub files: Vec<MediaFile>,
    pub schedules: Vec<Schedule>,
    pub status: Status,
    pub file_id_gen: IdGenerator,
    pub schedule_id_gen: IdGenerator,
}

impl Default for State {
    fn default() -> Self {
        State {
            files: vec![],
            schedules: vec![],
            status: Status::Init,
            file_id_gen: IdGenerator::new(0),
            schedule_id_gen: IdGenerator::new(0),
        }
    }
}

impl State {
    pub fn load() -> State {
        let files = load_media_files();
        let schedules = load_schedules();
        State {
            file_id_gen: IdGenerator::new(files.iter().map(|f| f.id).max().unwrap_or(0)),
            schedule_id_gen: IdGenerator::new(schedules.iter().map(|s| s.id).max().unwrap_or(0)),
            files,
            schedules,
            status: Status::Idle,
        }
    }

    fn save_media(&self) {
        write_media_files(&self.files);
    }

    pub fn save_schedules(&self) {
        write_schedules(&self.schedules);
    }

    pub fn get_media(&self, id: u32) -> Option<&MediaFile> {
        self.files.iter().find(|f| f.id == id)
    }

    pub fn add_media(&mut self, name: String, path: String) {
        self.files
            .push(MediaFile::new(self.file_id_gen.next(), name, path));
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
            self.schedule_id_gen.next(),
            file_id,
            schedule,
        ));
        self.save_schedules();
    }

    pub fn edit_schedule(&mut self, id: u32, file_id: u32, schedule: String) {
        let mut sched = self.get_mut_schedule(id).unwrap();
        if file_id == sched.file_id && schedule == sched.schedule {
            return;
        }
        sched.file_id = file_id;
        sched.schedule = schedule;
        self.save_schedules();
    }

    pub fn remove_schedule(&mut self, id: u32) {
        self.schedules.retain(|s| s.id != id);
        self.save_schedules();
    }
}

impl MediaFile {
    pub fn new(id: u32, name: String, path: String) -> MediaFile {
        MediaFile { id, name, path }
    }
}
