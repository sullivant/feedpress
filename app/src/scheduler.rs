pub mod scheduler {
    
	use chrono::{Local, Utc};
	use cron::Schedule;
	use std::str::FromStr;
	use tokio::task;
	use tokio::time::sleep;

    use crate::{create_edition, get_config};

	pub struct Scheduler {
		pub running: bool,
	}
	
	impl Scheduler {
		pub fn new() -> Scheduler {
			Scheduler {
				running: false,
			}
		}

		pub async fn run(&mut self) {
			info!("Starting scheduled pressing processor...");
			let config = get_config().unwrap();
			if !config.schedule_enabled {
				info!("Schedule is not enabled; Not going to worry about it.");
				return;
			} 

			info!("Attempting to use schedule of: {}",config.schedule);
			let schedule = match Schedule::from_str(&config.schedule){
				Ok(s) => s,
				Err(_) => {warn!("Failed to parse CRON expression, not running schedule.");
					return;
				},
			};

			let next = schedule.upcoming(Utc).next().unwrap();
			println!("Now: {}", Utc::now());
			println!("Next press: {}", next);

		}
	}

	
	
	
}