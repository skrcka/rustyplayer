use hyper::StatusCode;
use std::collections::HashMap;
use std::convert::Infallible;
use warp::multipart::form;
use warp::{any, body, get, path, post, Filter, Rejection, Reply};

use crate::consts::MEDIA_PATH;
use crate::consts::WEB_PATH;
use crate::handlers;
use crate::PlayerMutex;
use crate::SchedulerMutex;
use crate::StateMutex;

pub fn routes(
    state: StateMutex,
    player: PlayerMutex,
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    serve_web()
        .or(serve_files())
        .or(get_status(state.clone(), player.clone()))
        .or(get_schedules(state.clone()))
        .or(get_files(state.clone()))
        .or(upload_files(state.clone()))
        .or(delete_file(state.clone(), scheduler.clone()))
        .or(download_file(state.clone()))
        .or(stop(state.clone(), player.clone()))
        .or(play(state.clone(), player.clone()))
        .or(pause(state.clone(), player.clone()))
        .or(resume(state.clone(), player))
        .or(add_schedule(state.clone()))
        .or(edit_schedule(state.clone(), scheduler.clone()))
        .or(remove_schedule(state, scheduler.clone()))
        .or(activate(scheduler.clone()))
        .or(deactivate(scheduler))
}

fn serve_web() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end()
        .and(get())
        .and(warp::fs::dir(WEB_PATH).recover(handle_rejection))
}

fn serve_files() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("export")
        .and(get())
        .and(warp::fs::dir(MEDIA_PATH).recover(handle_rejection))
}

async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    println!("Rejection: {:?}", err);
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
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("status")
        .and(get())
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::get_status)
}

fn get_files(state: StateMutex) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
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
        .and(form())
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

fn resume(
    state: StateMutex,
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("resume")
        .and(get())
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::resume)
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

fn edit_schedule(
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("reschedule")
        .and(post())
        .and(json_edit_schedule())
        .and(with_state(state))
        .and(with_scheduler(scheduler))
        .and_then(handlers::edit_schedule)
}

fn add_schedule(
    state: StateMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("schedule")
        .and(post())
        .and(json_schedule())
        .and(with_state(state))
        .and_then(handlers::add_schedule)
}

fn remove_schedule(
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("remove")
        .and(get())
        .and(with_id())
        .and(with_state(state))
        .and(with_scheduler(scheduler))
        .and_then(handlers::remove_schedule)
}

fn activate(
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("activate")
        .and(get())
        .and(with_id())
        .and(with_scheduler(scheduler))
        .and_then(handlers::activate)
}

fn deactivate(
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    path("deactivate")
        .and(get())
        .and(with_id())
        .and(with_scheduler(scheduler))
        .and_then(handlers::deactivate)
}

fn with_state(
    state: StateMutex,
) -> impl Filter<Extract = (StateMutex,), Error = Infallible> + Clone {
    any().map(move || state.clone())
}

fn with_scheduler(
    scheduler: SchedulerMutex,
) -> impl Filter<Extract = (SchedulerMutex,), Error = Infallible> + Clone {
    any().map(move || scheduler.clone())
}

fn with_stream(
    player: PlayerMutex,
) -> impl Filter<Extract = (PlayerMutex,), Error = Infallible> + Clone {
    any().map(move || player.clone())
}

fn with_id() -> impl Filter<Extract = (u32,), Error = Rejection> + Clone {
    warp::query::<HashMap<String, String>>()
        .map(|query: HashMap<String, String>| {
            if let Some(id) = query.get("id") {
                match id.parse::<u32>() {
                    Ok(id) => Ok(id),
                    Err(_) => Err(warp::reject()),
                }
            } else {
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

fn json_schedule() -> impl Filter<Extract = ((u32, String),), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

fn json_edit_schedule() -> impl Filter<Extract = ((u32, u32, String),), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}
