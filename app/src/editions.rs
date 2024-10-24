pub mod editions {
    use serde::{Deserialize, Serialize};

	/// This struct carries with it a list of all editions found, contains
	/// a vector of [EditionEntry] 
	#[derive(Debug, Serialize)]
	pub struct Editions {
		pub editions: Vec<EditionEntry>,
	}
	
	/// A single edition made up of two basic points of data, a name (file) and date (creation)
	#[derive(Debug, Serialize, Deserialize)]
	pub struct EditionEntry {
		pub name: String,
		pub date: String,
		pub size: String,
		pub pages: usize,
		pub img: String,
	}
}