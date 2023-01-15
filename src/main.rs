use std::fs::File;
use std::io::{BufReader, Write, self};
use std::{thread, fs};
use std::sync::Arc;
use rodio::OutputStreamHandle;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};
use warp::Filter;
use std::time::{Instant};
//use local_ip_address::local_ip;
use tokio_cron_scheduler::{JobScheduler, JobToRun, Job};
use chrono::prelude::*;
use rodio::{Decoder, OutputStream, source::Source};

mod routes;
mod handlers;
mod models;
mod utils;

use utils::play_file;
use utils::load_media_files;
use utils::load_schedules;
use models::Activity;
use models::ActiveSchedule;


pub type StateMutex = Arc<Mutex<models::State>>;
pub type StreamMutex = Arc<Mutex<OutputStreamHandle>>;

#[tokio::main]
async fn main() {
    let mut state = models::State::load();

    let sched = JobScheduler::new().await.unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut active_schedules: Vec<ActiveSchedule> = Vec::new();
    let media_files = state.files.clone();
    let schedules = state.schedules.clone();
    for schedule in schedules.iter().filter(move |s| s.activity == Activity::Active) {
        let mediafile = media_files.iter().find(|f| f.id == schedule.file_id).unwrap().clone();
        let stream= stream_handle.clone();
        let job = Job::new(schedule.schedule.as_str(), move |_uuid, _l| {
            play_file(&mediafile, &stream);
        }).unwrap();
        active_schedules.push(ActiveSchedule{id: 0, schedule_id: schedule.id, job: job.clone()});
        sched.add(job).await.unwrap();
    }

    sched.start().await.unwrap();

    let statepointer : StateMutex = Arc::new(Mutex::new(state));
    let streammutex : StreamMutex = Arc::new(Mutex::new(stream_handle));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(statepointer.clone(), streammutex.clone()).with(cors);
    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
