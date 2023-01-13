use std::convert::Infallible;

use warp::{self, http::StatusCode};
use crate::StateMutex;


pub async fn get_status(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&*state))
}

pub async fn get_files(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&*state))
}

pub async fn stop(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    Ok(StatusCode::OK)
}

/*
pub async fn update_status(
    content: (i32, bool, f64, i32, f64, i32),
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let (mode, pull, volume, volume_unit, time_rate, time_rate_unit) = content;
    if volume_unit == 0 {
    }
    else if volume_unit == 1 {
        state.ml = volume / 1000.0;
    }
    else if volume_unit == 2 {
        state.ml = volume / 1_000_000.0;
    }
    state.steps = (state.ml * state.steps_per_ml as f64) as i32;
    if mode == 1 {
        if time_rate_unit == 0 {
            state.time_rate = time_rate;
        }
        else if time_rate_unit == 1 {
            state.time_rate = time_rate * 60.0;
        }
    }
    else if mode == 3 {
        if time_rate_unit == 0 {
            state.time_rate = time_rate;
        }
        else if time_rate_unit == 1 {
            state.time_rate = time_rate / 1000.0;
        }
        else if time_rate_unit == 2 {
            state.time_rate = time_rate / 1_000_000.0;
        }
    }
    state.pull = pull;
    state.mode = mode;
    state.running = true;

    Ok(StatusCode::OK)
}
*/

pub async fn pause(
    state: StateMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    state.pause = true;

    Ok(StatusCode::OK)
}
