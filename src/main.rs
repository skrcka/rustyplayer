use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use tokio_cron_scheduler::{JobScheduler, Job};
use rodio::OutputStream;

mod routes;
mod handlers;
mod models;
mod utils;
mod player;
mod consts;

use models::Activity;
use models::ActiveSchedule;
use player::Player;

pub type StateMutex = Arc<Mutex<models::State>>;
pub type PlayerMutex = Arc<Mutex<Player>>;
pub type ScheduleMutex = Arc<Mutex<Vec<ActiveSchedule>>>;
pub type SchedulerMutex = Arc<Mutex<JobScheduler>>;


#[tokio::main]
async fn main() {
    let state = models::State::load();

    let sched = JobScheduler::new().await.unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let player: Player = Player::new(&stream_handle);
    let playermutex : PlayerMutex = Arc::new(Mutex::new(player));
    let active_schedules: Vec<ActiveSchedule> = Vec::new();
    let scheduler_mutex: SchedulerMutex = Arc::new(Mutex::new(sched));
    let schedule_mutex: ScheduleMutex= Arc::new(Mutex::new(active_schedules));

    let media_files = state.files.clone();
    let schedules = state.schedules.clone();
    for schedule in schedules.iter().filter(move |s| s.activity == Activity::Active) {
        let mediafile = media_files.iter().find(|f| f.id == schedule.file_id).unwrap().clone();
        let pm = playermutex.clone();
        let job = Job::new(schedule.schedule.as_str(), move |_uuid, _l| {
            let player = pm.clone();
            let media = mediafile.clone();
            tokio::spawn(async move {
                let player = player.lock().await;
                player.play(&media);
            });
        }).unwrap();
        schedule_mutex.lock().await.push(ActiveSchedule{id: 0, schedule_id: schedule.id, job: job.clone()});
        sched.add(job).await.unwrap();
    }

    sched.start().await.unwrap();

    let statepointer : StateMutex = Arc::new(Mutex::new(state));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(statepointer.clone(),
                                                                                         playermutex.clone(), 
                                                                                         schedule_mutex.clone(),
                                                                                         scheduler_mutex.clone(),
                                                                                        ).with(cors);
    warp::serve(routes)
        .run(([0, 0, 0, 0], 5000))
        .await;
}
