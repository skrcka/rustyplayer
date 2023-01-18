use std::path::Path;
use std::{fs::File, io::BufReader};

use crate::models::{MediaFile, Schedule};
use crate::consts::RESOURCE_PATH;
use crate::consts::MEDIA_PATH;


pub fn write_media_files(files: &Vec<MediaFile>) {
    let path = Path::new(".").join(RESOURCE_PATH).join("media.json");
    print!("writing media files to: {}", path.display());
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, files).unwrap();
}

pub fn load_media_files() -> Vec<MediaFile> {
    let path = Path::new(".").join(RESOURCE_PATH).join("media.json");
    println!("loading media files from: {}", path.display());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<MediaFile> = serde_json::from_reader(reader).unwrap();
    files
}

pub fn write_schedules(schedules: &Vec<Schedule>) {
    let path = Path::new(".").join(RESOURCE_PATH).join("schedules.json");
    println!("writing schedules to: {}", path.display());
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, schedules).unwrap();
}

pub fn load_schedules() -> Vec<Schedule> {
    let path = Path::new(".").join(RESOURCE_PATH).join("schedules.json");
    println!("loading schedules from: {}", path.display());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<Schedule> = serde_json::from_reader(reader).unwrap();
    files
}

pub async fn write_file(file_name: &str, file_ending: &str, data: &Vec<u8>) {
    let path = format!("{}{}.{}", MEDIA_PATH, file_name, file_ending);
    println!("writing file {} to: {}", file_name, path);
    tokio::fs::write(&path, data).await.map_err(|e| {
        eprint!("error writing file: {}", e);
        warp::reject::reject()
    }).unwrap();
    println!("created file: {}", file_name);
}
