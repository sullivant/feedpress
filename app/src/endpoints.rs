pub mod endpoints {
	use chrono::prelude::*;
	use rocket::serde::json::Json;
	use std::fs;
	use std::fs::File;
	use std::io::Write;
	use std::process::Command;


	use crate::config::config::FeedConfig;
	use crate::editions::editions::{EditionEntry, Editions};
	use crate::get_config;


	#[get("/edition")]
	pub fn api_get_edition_list() -> Json<Editions> {
		let mut edition_list: Editions = Editions {
			editions: Vec::new(),
		};

		for file in fs::read_dir("../output/").unwrap() {
			let tf = file.unwrap();
			
			//let this_date: String = format!("{:?}",&tf.metadata().unwrap().created().unwrap());
			let datetime: DateTime<Utc> = tf.metadata().unwrap().created().unwrap().into();

			let this_entry = EditionEntry{
				name: tf.path().file_name().unwrap().to_str().unwrap().to_string(),
				date: format!("{}", datetime.format("%Y/%m/%d")),
			};
			edition_list.editions.push(this_entry);
		}

		Json(edition_list)
	}


	#[get("/config")]
	pub fn api_get_config() -> Json<FeedConfig> {
		Json(get_config().unwrap())
	}

	#[post("/config", format = "json", data = "<config>")]
	pub fn api_update_config(config: Json<FeedConfig>) {
		let toml = toml::to_string(&config.0).unwrap();

		// Write this updated config to a file
		let mut file = File::create("../data/config.toml").unwrap();
		file.write_all(toml.as_bytes()).unwrap();

	}

	#[post("/press")]
	pub fn api_press_edition() -> Json<Editions> {
		let local_time: DateTime<Local> = Local::now();
		let filename = format!("{}", local_time.format("%Y%m%d"));

		println!("Pressing new edition with filename: {}.", &filename);

		// Sample command: 
		// typst compile templates/feedpress.typ output/feedpress.pdf --root ./
		
		let output = Command::new("sh")
		.arg("-c")
		.arg(format!("typst compile ../templates/feedpress.typ ../output/{}.pdf --root ../",&filename))
        .output()
        .expect("Failed to execute command");

		println!("Executed compile: {:?}", output.stdout);

		api_get_edition_list()
	}

}