use std::collections::HashMap;
use std::convert::Infallible;
use hyper::StatusCode;
use warp::{
    Filter, Rejection, Reply, body, multipart::form,
    path, get, any, query, post, delete,
};

use crate::StateMutex;
use crate::PlayerMutex;
use crate::SchedulerMutex;
use crate::handlers;
use crate::consts::WEB_PATH;

pub fn routes(
    state: StateMutex,
    player: PlayerMutex,
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    serve_web()
        .or(get_status(state.clone()))
        .or(get_schedules(state.clone()))
        .or(get_files(state.clone()))
        .or(upload_files(state.clone()))
        .or(delete_file(state.clone(), scheduler.clone()))
        .or(download_file(state.clone()))
        .or(stop(state.clone(), player.clone()))
        .or(play(state.clone(), player.clone()))
        .or(pause(state.clone(), player.clone()))
}

fn serve_web() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end()
        .and(get())
        .and(warp::fs::dir(WEB_PATH)
        .recover(handle_rejection))
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string())
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        (StatusCode::BAD_REQUEST, "Payload too large".to_string())
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )
    };

    Ok(warp::reply::with_status(message, code))
}

fn get_status(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("status")
        .and(get())
        .and(with_state(state))
        .and_then(handlers::get_status)
}

fn get_files(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("files")
        .and(get())
        .and(with_state(state))
        .and_then(handlers::get_files)
}

fn get_schedules(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("schedules")
        .and(get())
        .and(with_state(state))
        .and_then(handlers::get_schedules)
}

fn upload_files(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("upload")
        .and(post())
        .and(form().max_length(5_000_000))
        .and(with_state(state))
        .and_then(handlers::upload_files)
}

fn delete_file(
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path!("delete")
        .and(get())
        .and(with_id())
        .and(with_state(state))
        .and(with_scheduler(scheduler))
        .and_then(handlers::delete_file)
}

fn download_file(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("download")
        .and(get())
        .and(with_id())
        .and(with_state(state))
        .and_then(handlers::download_file)
}

fn pause(
    state: StateMutex,
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("pause")
        .and(get())
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::pause)
}

fn stop(
    state: StateMutex,
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("stop")
        .and(get())
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::stop)
}

fn play(
    state: StateMutex,
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("play")
        .and(get())
        .and(with_id())
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::play)
}

fn with_state(state: StateMutex) -> impl Filter<Extract = (StateMutex,), Error = Infallible> + Clone {
    any().map(move || state.clone())
}

fn with_scheduler(scheduler: SchedulerMutex) -> impl Filter<Extract = (SchedulerMutex,), Error = Infallible> + Clone {
    any().map(move || scheduler.clone())
}

fn with_stream(player: PlayerMutex) -> impl Filter<Extract = (PlayerMutex,), Error = Infallible> + Clone {
    any().map(move || player.clone())
}

fn with_id() -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
    warp::query::<HashMap<String, String>>()
        .map(| query: HashMap<String, String> | {
            if let Some(id) = query.get("id") {
                match id.parse::<u32>() {
                    Ok(id) => Ok(id),
                    Err(_) => Err(warp::reject()),
                }
            }
            else {
                Err(warp::reject())
            }
        })
        .and_then(|id: Result<u32, Rejection>| async move {
            match id {
                Ok(id) => Ok(id),
                Err(_) => Err(warp::reject()),
            }
        })
}

fn json_body() -> impl Filter<Extract = ((i32, bool, f64, i32, f64, i32),), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16)
    .and(body::json())
}
