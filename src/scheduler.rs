use tokio_cron_scheduler::{Job, JobScheduler};

use crate::models::{ActiveSchedule, Activity};
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

    pub async fn add(&mut self, schedule_id: u32) {
        let state = self.state.lock().await;
        let schedule = state.get_schedule(schedule_id).unwrap().clone();

        let player = self.player.clone();
        let media = state.get_media(schedule.file_id).unwrap().clone();
        let job = Job::new_async(schedule.schedule.as_str(), move |_uuid, _l| {
            let player = player.clone();
            let media = media.clone();
            Box::pin(async move {
                println!("Triggered schedule: {}", schedule.id);
                let media = media.clone();
                let player = player.lock().await;
                player.play(&media);
            })
        })
        .unwrap();
        self.active_schedules.push(ActiveSchedule {
            schedule_id: schedule.id,
            job_id: job.guid(),
        });
        self.scheduler.add(job).await.unwrap();

        drop(state);
        let mut state = self.state.lock().await;
        state
            .schedules
            .iter_mut()
            .find(|s| s.id == schedule_id)
            .unwrap()
            .activity = Activity::Active;
        println!("Added schedule: {} as active", schedule_id);
        state.save_schedules();
    }

    pub async fn remove(&mut self, id: u32) {
        println!("Removing schedule: {} from active", id);
        let active_schedule = self
            .active_schedules
            .iter()
            .find(|s| s.schedule_id == id)
            .unwrap();
        self.scheduler
            .remove(&active_schedule.job_id)
            .await
            .unwrap();
        println!("Removed schedule: {} from active", id);
        self.active_schedules.retain(|s| s.schedule_id != id);
        let mut state = self.state.lock().await;
        state
            .schedules
            .iter_mut()
            .find(|s| s.id == id)
            .unwrap()
            .activity = Activity::Inactive;
        state.save_schedules();
    }

    pub async fn reschedule(&mut self, id: u32) {
        self.remove(id).await;
        self.add(id).await;
    }

    pub async fn load(&mut self) {
        println!("Loading schedules");
        let schedules = self.state.lock().await.schedules.clone();
        for schedule in schedules
            .iter()
            .filter(move |s| s.activity == Activity::Active)
        {
            self.add(schedule.id).await;
        }
    }

    pub async fn start(&mut self) {
        println!("Starting scheduler");
        self.scheduler.start().await.unwrap();
    }

    pub async fn stop(&mut self) {
        println!("Stopping scheduler");
        self.scheduler.shutdown().await.unwrap();
    }
}
