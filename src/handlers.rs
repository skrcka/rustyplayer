use bytes::BufMut;
use futures::TryStreamExt;
use hyper::Uri;
use std::convert::Infallible;
use warp::multipart::{FormData, Part};
use warp::{self, http::StatusCode, reject::Reject, Rejection};

use crate::models::{Activity, Schedule, Status};
use crate::utils::remove_file;
use crate::utils::write_file;
use crate::PlayerMutex;
use crate::{SchedulerMutex, StateMutex};

#[derive(Debug)]
struct InvalidFile;
impl Reject for InvalidFile {}

pub async fn get_status(
    state: StateMutex,
    player: PlayerMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    if state.status == Status::Running {
        let player = player.lock().await;
        if player.done() {
            state.status = Status::Idle;
        }
    }
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
    player: PlayerMutex,
) -> Result<impl warp::Reply, Infallible> {
    let mut state = state.lock().await;
    let player = player.lock().await;
    state.status = Status::Running;
    player.play(state.get_media(id).unwrap());
    Ok(StatusCode::OK)
}

pub async fn upload_files(
    mut form: FormData,
    state: StateMutex,
) -> Result<impl warp::Reply, Rejection> {
    while let Some(field) = form.try_next().await.map_err(|e| {
        eprintln!("form error during part processing: {}", e);
        warp::reject::reject()
    })? {
        let p: Part = field;
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending = match content_type {
                Some(file_type) => match file_type {
                    "audio/mp3" | "audio/mpeg" | "audio" => "mp3",
                    "audio/ogg" => "ogg",
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            };

            let file_name = match p.filename() {
                Some(filename) => filename.to_string(),
                None => {
                    eprintln!("file name could not be determined");
                    return Err(warp::reject::reject());
                }
            };

            let file_name = file_name
                .strip_suffix(format!(".{}", file_ending).as_str())
                .ok_or_else(|| {
                    eprintln!("failed to strip file extension");
                    warp::reject::reject()
                })?;

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
                })?;

            let path = write_file(file_name, file_ending, &value).await;

            let mut state = state.lock().await;
            state.add_media(file_name.to_string(), path);
        }
    }

    Ok(StatusCode::OK)
}

pub async fn delete_file(
    id: u32,
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> Result<impl warp::Reply, Rejection> {
    let mut state = state.lock().await;
    let mut scheduler = scheduler.lock().await;
    let file_locator = state.get_media(id).unwrap().path.clone();
    remove_file(file_locator.as_str()).await;
    state.remove_media(id);
    let schedules_to_disable = state
        .schedules
        .iter()
        .filter(|s| s.file_id == id)
        .filter(|s| s.activity == Activity::Active)
        .collect::<Vec<&Schedule>>();
    for s in schedules_to_disable.iter() {
        scheduler.remove(s.id).await;
    }
    Ok(StatusCode::OK)
}

pub async fn download_file(id: u32, state: StateMutex) -> Result<impl warp::Reply, Rejection> {
    let state = state.lock().await;
    let file_name = state.get_media(id).unwrap().name.clone();
    println!("redirrecting to download file: {}", file_name);
    let url = format!("/export/{}", file_name);
    let uri = url.parse::<Uri>().expect("valid URI");
    Ok(warp::redirect(uri))
}

pub async fn add_schedule(
    content: (u32, String),
    state: StateMutex,
) -> Result<impl warp::Reply, Rejection> {
    let (file_id, schedule) = content;
    let mut state = state.lock().await;
    state.add_schedule(file_id, schedule);
    Ok(StatusCode::OK)
}

pub async fn edit_schedule(
    content: (u32, u32, String),
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> Result<impl warp::Reply, Rejection> {
    let (id, file_id, schedule) = content;
    let mut state = state.lock().await;
    state.edit_schedule(id, file_id, schedule);
    if state.get_schedule(id).unwrap().activity == Activity::Active {
        let mut scheduler = scheduler.lock().await;
        scheduler.reschedule(id).await;
    }
    Ok(StatusCode::OK)
}

pub async fn remove_schedule(
    id: u32,
    state: StateMutex,
    scheduler: SchedulerMutex,
) -> Result<impl warp::Reply, Rejection> {
    let mut state = state.lock().await;
    if state.get_schedule(id).unwrap().activity == Activity::Active {
        let mut scheduler = scheduler.lock().await;
        scheduler.remove(id).await;
    }
    state.remove_schedule(id);
    Ok(StatusCode::OK)
}

pub async fn activate(id: u32, scheduler: SchedulerMutex) -> Result<impl warp::Reply, Rejection> {
    let mut scheduler = scheduler.lock().await;
    scheduler.add(id).await;
    Ok(StatusCode::OK)
}

pub async fn deactivate(id: u32, scheduler: SchedulerMutex) -> Result<impl warp::Reply, Rejection> {
    let mut scheduler = scheduler.lock().await;
    scheduler.remove(id).await;
    Ok(StatusCode::OK)
}
