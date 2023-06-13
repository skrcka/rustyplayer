use std::path::Path;
use std::{fs::File, io::BufReader};

use crate::consts::MEDIA_PATH;
use crate::consts::RESOURCE_PATH;
use crate::models::{MediaFile, Schedule};

pub fn write_media_files(files: &Vec<MediaFile>) {
    let path = Path::new(RESOURCE_PATH).join("media.json");
    println!("writing media files to: {}", path.display());
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, files).unwrap();
}

pub fn load_media_files() -> Vec<MediaFile> {
    let path = Path::new(RESOURCE_PATH).join("media.json");
    println!("loading media files from: {}", path.display());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<MediaFile> = serde_json::from_reader(reader).unwrap();
    files
}

pub fn write_schedules(schedules: &Vec<Schedule>) {
    let path = Path::new(RESOURCE_PATH).join("schedules.json");
    println!("writing schedules to: {}", path.display());
    let file = File::create(path).unwrap();
    serde_json::to_writer(file, schedules).unwrap();
}

pub fn load_schedules() -> Vec<Schedule> {
    let path = Path::new(RESOURCE_PATH).join("schedules.json");
    println!("loading schedules from: {}", path.display());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let files: Vec<Schedule> = serde_json::from_reader(reader).unwrap();
    files
}

pub async fn write_file(file_name: &str, file_ending: &str, data: &Vec<u8>) -> String {
    let path = Path::new(MEDIA_PATH)
        .join(file_name)
        .with_extension(file_ending);
    println!("writing file {} to: {}", file_name, path.display());
    tokio::fs::write(&path, data)
        .await
        .map_err(|e| {
            eprintln!("error writing file: {}", e);
            warp::reject::reject()
        })
        .unwrap();
    println!("created file: {}", file_name);
    path.to_string_lossy().to_string()
}

pub async fn remove_file(file_locator: &str) {
    let path = Path::new(file_locator);
    // delete file
    println!("deleting file: {}", path.display());
    tokio::fs::remove_file(&path)
        .await
        .map_err(|e| {
            eprint!("error deleting file: {}", e);
            warp::reject::reject()
        })
        .unwrap();
    println!("deleted file: {}", file_locator);
}
