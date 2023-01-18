use std::convert::Infallible;
use warp::{
    Filter, Rejection, Reply, body,
    path, get, any, query, post, multipart::form
};

use crate::StateMutex;
use crate::PlayerMutex;
use crate::handlers;


pub fn routes(
    state: StateMutex,
    player: PlayerMutex,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    get_status(state.clone())
        .or(get_schedules(state.clone()))
        .or(get_files(state.clone()))
        .or(upload_files(state.clone()))
        .or(stop(state.clone(), player.clone()))
        .or(play(state.clone(), player.clone()))
        .or(pause(state.clone(), player.clone()))
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
        .and(query().map(|id: u32| id))
        .and(with_state(state))
        .and(with_stream(player))
        .and_then(handlers::play)
}

fn with_state(state: StateMutex) -> impl Filter<Extract = (StateMutex,), Error = Infallible> + Clone {
    any().map(move || state.clone())
}

fn with_stream(player: PlayerMutex) -> impl Filter<Extract = (PlayerMutex,), Error = Infallible> + Clone {
    any().map(move || player.clone())
}

fn json_body() -> impl Filter<Extract = ((i32, bool, f64, i32, f64, i32),), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16)
    .and(body::json())
}
