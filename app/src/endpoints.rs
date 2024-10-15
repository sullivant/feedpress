pub mod endpoints {
	use chrono::prelude::*;
	use rocket::serde::json::Json;
	use std::fs::{self, DirEntry};
	use std::fs::File;
	use std::io::Write;
	use std::process::Command;


	use crate::config::config::FeedConfig;
	use crate::editions::editions::{EditionEntry, Editions};
	use crate::{get_config, press_feeds, VERSION};

	/// Returns a formatted string containing the file's creation date
	/// or "01011900" if unable to determine.
	pub fn get_file_create(file: &DirEntry) -> String {
		let tf_meta = match file.metadata() {
			Ok(m) => m,
			Err(_) => return "01011900".to_string(),
		};

		let created: DateTime<Utc> = match tf_meta.created() {
			Ok(c) => c.into(),
			Err(_) => return "01011900".to_string(),
		};

		format!("{}", created.format("%Y/%m/%d"))
	}

	/// Returns a formatted filename or the string "null" if unable
	/// to determine.
	pub fn get_file_name(file: &DirEntry) -> String {
		let binding = file.path();
  		let name = match binding.file_name() {
			Some(n) => n,
			None => return "null".to_string(),
		};

		name.to_str().unwrap_or("null").to_string()
	}

	/// GET request will return a version string, located in the
	/// env variable [VERSION]
	#[get("/version")]
	pub fn api_get_version() -> String {
		VERSION.to_string()
	}

	#[get("/edition")]
	pub fn api_get_edition_list() -> Json<Editions> {
		let mut edition_list: Editions = Editions {
			editions: Vec::new(),
		};

		for file in fs::read_dir("../output/").unwrap() {
			let tf = file.unwrap();

			let create_string: String = get_file_create(&tf);
			let name_string: String = get_file_name(&tf);
			
			if !name_string.ends_with("pdf") {
				continue;
			}

			let this_entry = EditionEntry{
				name: name_string,
				date: create_string,
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

	#[delete("/press", format = "json", data = "<edition>")]
	pub fn api_remove_edition(edition: Json<EditionEntry>) -> Json<Editions> {
		if edition.name.ends_with("pdf") {
			let this_path = format!("../output/{}", edition.name);
			println!("Removing edition: {}, dated: {}, at: {}", edition.name, edition.date, this_path);
			fs::remove_file(this_path).unwrap();
		}

		api_get_edition_list()
	}

	#[post("/press")]
	pub async fn api_press_edition() -> Json<Editions> {
		let local_time: DateTime<Local> = Local::now();
		let filename = format!("{}", local_time.format("%Y%m%d"));

		println!("Pressing new edition with filename: {}.", &filename);

		// Press our feeds first to create a new input file..
		press_feeds().await;

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