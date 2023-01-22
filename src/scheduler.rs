use tokio_cron_scheduler::{JobScheduler, Job};

use crate::models::{Status, Activity, Schedule, ActiveSchedule, State};
use crate::PlayerMutex;
use crate::StateMutex;


pub struct Scheduler {
    scheduler: JobScheduler,
    active_schedules: Vec<ActiveSchedule>,
    player: PlayerMutex,
    state: StateMutex,
}

impl Scheduler {
    pub async fn new(player: PlayerMutex, state: StateMutex) -> Scheduler {
        Scheduler {
            scheduler: JobScheduler::new().await.unwrap(),
            active_schedules: vec![],
            player,
            state,
        }
    }

    pub async fn add(&mut self, schedule: Schedule) {
        println!("Adding schedule: {:?} as active", schedule);
        let player = self.player.clone();
        let state = self.state.lock().await;
        let media = state.get_media(schedule.file_id).unwrap().clone();
        let job = Job::new(schedule.schedule.as_str(), move |_uuid, _l| {
            println!("Triggered schedule: {}", schedule.id);
            let player = player.clone();
            let media = media.clone();
            tokio::spawn(async move {
                let player = player.lock().await;
                player.play(&media);
            });
        }).unwrap();
        self.active_schedules.push(ActiveSchedule{schedule_id: schedule.id, job: job.clone()});
        self.scheduler.add(job).await.unwrap();
    }

    pub async fn remove(&mut self, id: u32) {
        let active_schedule = self.active_schedules.iter().find(|s| s.schedule_id == id).unwrap();
        self.scheduler.remove(&active_schedule.job.guid()).await.unwrap();
        self.active_schedules.retain(|s| s.schedule_id != id);
        self.state.lock().await.schedules.iter_mut().find(|s| s.id == id).unwrap().activity = Activity::Inactive;
    } 
    pub async fn load(&mut self) {
        let state = self.state.lock().await.clone();
        for schedule in state.schedules.iter().filter(move |s| s.activity == Activity::Active) {
            self.add(schedule.clone()).await;
        }
    }

    pub async fn start(&mut self) {
        println!("Starting scheduler");
        self.scheduler.start().await.unwrap();
    }
}
