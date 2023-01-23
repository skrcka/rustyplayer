use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;
use rodio::OutputStream;

mod routes;
mod handlers;
mod models;
mod utils;
mod player;
mod consts;
mod scheduler;

use player::Player;
use scheduler::Scheduler;

pub type StateMutex = Arc<Mutex<models::State>>;
pub type PlayerMutex = Arc<Mutex<Player>>;
pub type SchedulerMutex = Arc<Mutex<Scheduler>>;


const PORT: u16 = 5000;

#[tokio::main]
async fn main() {
    let state = models::State::load();
    let statemutex : StateMutex = Arc::new(Mutex::new(state));

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let player: Player = Player::new(&stream_handle);
    let playermutex : PlayerMutex = Arc::new(Mutex::new(player));
    
    let mut scheduler = Scheduler::new(playermutex.clone(), statemutex.clone()).await;
    scheduler.load().await;
    scheduler.start().await;
    let scheduler_mutex: SchedulerMutex = Arc::new(Mutex::new(scheduler));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["User-Agent", "content-type", "Sec-Fetch-Mode", "Referer", "Origin", "Access-Control-Request-Method", "Access-Control-Request-Headers", "Access-Control-Allow-Origin"])
        .allow_methods(vec!["POST", "GET"]);
    let routes = routes::routes(statemutex.clone(),
                                                                                         playermutex.clone(),
                                                                                         scheduler_mutex.clone(),
                                                                                        ).with(cors);

    println!("Starting server on port {}", PORT);
    println!("http://127.0.0.1:{}/", PORT);
    warp::serve(routes)
        .run(([0, 0, 0, 0], PORT))
        .await;
}
