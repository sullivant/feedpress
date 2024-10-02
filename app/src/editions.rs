pub mod editions {
    use serde::Serialize;

	#[derive(Debug, Serialize)]
	pub struct Editions {
		pub editions: Vec<EditionEntry>,
	}
	
	#[derive(Debug, Serialize)]
	pub struct EditionEntry {
		pub name: String,
		pub date: String,
	}
}