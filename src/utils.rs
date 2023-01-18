use std::{fs::File, io::BufReader};

use crate::models::{MediaFile, Schedule};
use crate::consts::RESOURCE_PATH;
use crate::consts::MEDIA_PATH;


pub fn write_media_files(files: &Vec<MediaFile>) {
    let path = format!("{}media.json", RESOURCE_PATH);
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, files).unwrap();
}

pub fn load_media_files() -> Vec<MediaFile> {
    let path = format!("{}media.json", RESOURCE_PATH);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<MediaFile> = serde_json::from_reader(reader).unwrap();
    files
}

pub fn write_schedules(schedules: &Vec<Schedule>) {
    let path = format!("{}schedules.json", RESOURCE_PATH);
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, schedules).unwrap();
}

pub fn load_schedules() -> Vec<Schedule> {
    let path = format!("{}scedules.json", RESOURCE_PATH);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<Schedule> = serde_json::from_reader(reader).unwrap();
    files
}

pub async fn write_file(file_name: &str, file_ending: &str, data: &Vec<u8>) {
    let path = format!("{}{}.{}", MEDIA_PATH, file_name, file_ending);
    tokio::fs::write(&path, data).await.map_err(|e| {
        eprint!("error writing file: {}", e);
        warp::reject::reject()
    }).unwrap();
    println!("created file: {}", file_name);
}
