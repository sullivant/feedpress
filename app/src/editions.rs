pub mod editions {
    use serde::{Deserialize, Serialize};

	#[derive(Debug, Serialize)]
	pub struct Editions {
		pub editions: Vec<EditionEntry>,
	}
	
	#[derive(Debug, Serialize, Deserialize)]
	pub struct EditionEntry {
		pub name: String,
		pub date: String,
	}
}