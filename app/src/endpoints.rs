use chrono::prelude::*;
use rocket::serde::json::Json;
use std::fs::{self, DirEntry};
use std::fs::File;
use std::io::Write;
use lopdf::Document;

use crate::config::FeedConfig;
use crate::editions::{EditionEntry, Editions};
use crate::{create_edition, get_config, VERSION};

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

	format!("{}", created.format("%Y/%m/%d %H:%M:%S UTC"))
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

/// Returns the image file that corresponds to the input file. 
/// (strips the extension and path and then just returns it as <name>.png)
pub fn get_img_name(file: &DirEntry) -> String {
	let binding = file.path();
	let binding = binding.with_extension("png");
	let img_name = match binding.file_name() {
		Some(o) => o.to_str().unwrap_or(""),
		None => return "".to_string(),
	};

	img_name.to_string()
}

/// Returns a formatted string containing a plain english representation
/// of a file's size. (eg: "125k")
pub fn get_file_size(file: &DirEntry) -> String {
	let metadata = match file.metadata() {
		Ok(m) => m,
		Err(_) => return "0kb".to_string(),
	};

	format!("{}k",metadata.len()/1000)
}

/// Returns a simple numeric value of the pages in the PDF, based on what
/// lopdf gives us from its metadata
pub fn get_file_pages(file: &DirEntry) -> usize {
	let binding = file.path();
	let path = binding.as_os_str();

	let doc = match Document::load(path) {
		Ok(d) => d,
		Err(_) => return 0,
	};

	doc.get_pages().len()
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
		let size_string: String = get_file_size(&tf);
		let page_count: usize = get_file_pages(&tf);
		let img_string: String = get_img_name(&tf);
		
		if !name_string.ends_with("pdf") {
			continue;
		}

		let this_entry = EditionEntry{
			name: name_string,
			date: create_string,
			size: size_string,
			pages: page_count,
			img: img_string,
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
	if edition.name.ends_with(".pdf") {
		let this_path = format!("../output/{}", edition.name);

		// Make a PNG version of this filename ( remove '.pdf' )
		let stripped: String = edition.name.chars().take(edition.name.chars().count() - 4).collect();
		let this_img = format!("../output/{}.png", stripped);

		info!("Removing edition: {} (img: {}), dated: {}, at: {}", edition.name, this_img, edition.date, this_path);
		fs::remove_file(this_path).unwrap();
		fs::remove_file(this_img).unwrap();
	}

	api_get_edition_list()
}

#[post("/press")]
pub async fn api_press_edition() -> Json<Editions> {
	// Create an edition
	create_edition().await;
	
	// Refresh our edition list.
	api_get_edition_list()
}
