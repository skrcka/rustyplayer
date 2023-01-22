use tokio_cron_scheduler::{JobScheduler, Job};

use crate::models::{Status, Activity, Schedule, ActiveSchedule, State};
use crate::player::Player;


struct Scheduler {
    scheduler: JobScheduler,
    active_schedules: Vec<ActiveSchedule>,
    player: PlayerMutex,
    player: StateMutex,
}

impl Scheduler {
    pub fn new(player: PlayerMutex, state: StateMutex) -> Scheduler {
        Scheduler {
            scheduler: JobScheduler::new(),
            active_schedules: vec![],
            player,
            state,
        }
    }

    pub fn add(&mut self, schedule: Schedule) {
        let job = Job::new(schedule.schedule.as_str(), move |_uuid, _l| {
            let player = self.player.clone();
            let state = self.state.lock().await;
            let media = state.files.get_media(schedule.file_id).clone();
            tokio::spawn(async move {
                let player = player.lock().await;
                player.play(&media);
            });
        }).unwrap();
        self.active_schedules.push(ActiveSchedule{id: 0, schedule_id: schedule.id, job: job.clone()});
        self.scheduler.add(job).await.unwrap();
    }
}
