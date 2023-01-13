use std::fs::File;
use std::io::BufReader;
use std::{thread, fs};
use std::sync::Arc;
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


pub type StateMutex = Arc<Mutex<models::State>>;

#[tokio::main]
async fn main() {
    let sched = JobScheduler::new().await.unwrap();
    
    let jj = Job::new_repeated(Duration::from_secs(8), |_uuid, _l| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("/home/skrcka/police_s.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
    }).unwrap();
    sched.add(jj).await.unwrap();

    let state = models::State::new();

    let statepointer : StateMutex = Arc::new(Mutex::new(state));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(statepointer.clone()).with(cors);

    sched.start().await.unwrap();
    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
