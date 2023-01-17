use std::{fs::File, io::{BufReader, self, Write}};
use rodio::{OutputStreamHandle, Decoder, Source};

use crate::models::{MediaFile, Schedule};


pub fn write_media_files(files: &Vec<MediaFile>) {
    let mut file = File::create("resource/media.json").unwrap();
    serde_json::to_writer(file, files).unwrap();
}

pub fn load_media_files() -> Vec<MediaFile> {
    let mut file = File::open("resource/media.json").unwrap();
    let reader = BufReader::new(file);
    let mut files: Vec<MediaFile> = serde_json::from_reader(reader).unwrap();
    files
}

pub fn write_schedules(schedules: &Vec<Schedule>) {
    let mut file = File::create("resource/schedules.json").unwrap();
    serde_json::to_writer(file, schedules).unwrap();
}

pub fn load_schedules() -> Vec<Schedule> {
    let mut file = File::open("resource/schedules.json").unwrap();
    let reader = BufReader::new(file);
    let mut files: Vec<Schedule> = serde_json::from_reader(reader).unwrap();
    files
}

