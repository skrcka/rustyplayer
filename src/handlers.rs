use std::convert::Infallible;
use futures::TryStreamExt;
use warp::multipart::{FormData, Part};
use warp::{self, http::StatusCode, Rejection, reject::Reject};
use bytes::BufMut;

use crate::models::Status;
use crate::StateMutex;
use crate::PlayerMutex;
use crate::utils::write_file;


#[derive(Debug)]
struct InvalidFile;
impl Reject for InvalidFile {}

pub async fn get_status(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&state.status))
}

pub async fn get_files(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&state.files))
}

pub async fn get_schedules(state: StateMutex) -> Result<impl warp::Reply, Infallible> {
    let state = state.lock().await;
    Ok(warp::reply::json(&state.schedules))
}

pub async fn stop(state: StateMutex, player: PlayerMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let player = player.lock().await;
    player.stop();
    state.status = Status::Idle;
    Ok(StatusCode::OK)
}

pub async fn pause(state: StateMutex, player: PlayerMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let player = player.lock().await;
    player.pause();
    state.status = Status::Paused;
    Ok(StatusCode::OK)
}

pub async fn play(
    id: u32,
    state: StateMutex, 
    player: PlayerMutex) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let player = player.lock().await;
    state.status = Status::Running;
    player.play(state.get_media(id).unwrap());
    Ok(StatusCode::OK)
}

pub async fn upload_files(
    form: FormData,
    state: StateMutex,
) -> Result<impl warp::Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    }).unwrap();

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "audio/mp3" => {
                        file_ending = "mp3";
                    }
                    "audio/ogg" => {
                        file_ending = "ogg";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let filename = p.filename();
            let file_name;
            match filename {
                Some(filename) => {
                    file_name = filename.to_string();
                },
                None => {
                    eprintln!("file name could not be determined");
                    return Err(warp::reject::reject());
                }
            }
            let file_name = file_name.strip_suffix(file_ending).unwrap();

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                }).unwrap();

            write_file(file_name, file_ending, &value).await;

            let mut state = state.lock().await;
            state.add_media(file_name.to_string());
        }
    }

    Ok(StatusCode::OK)
}
