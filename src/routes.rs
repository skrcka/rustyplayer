use std::convert::Infallible;
use warp::{self, Filter};

use crate::StateMutex;
use crate::handlers;


pub fn routes(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_status(state.clone())
        .or(get_files(state.clone()))
        .or(pause(state.clone()))
}

fn get_status(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::get_status)
}

fn get_files(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("files")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::get_files)
}

/*
fn upload_files(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("upload")
        .and(warp::post())
        .and(json_body())
        .and(with_state(state))
        .and_then(handlers::update_files)
}
*/

fn pause(
    state: StateMutex,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("pause")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::pause)
}

fn with_state(state: StateMutex) -> impl Filter<Extract = (StateMutex,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

fn json_body() -> impl Filter<Extract = ((i32, bool, f64, i32, f64, i32),), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
    .and(warp::body::json())
}
