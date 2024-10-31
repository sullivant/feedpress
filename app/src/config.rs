pub mod config {
    use serde::{Deserialize, Serialize};

	/// Contains our application configuration.
	/// Configuration is written in the TOML format, seen most places.
	///
	/// Fields are not really optional but some will contain defaults
	/// when necessary.  This struct contains *global* configurations that
	/// can be overridden by individual feed entries.
	#[derive(Debug, Deserialize, Serialize)]
	pub struct FeedConfig {
		/// Controls if we show feed errors
		pub show_errors: bool,
		/// Max age, in days, we will consider an article acceptable to print
		#[serde(default)]
		pub max_age: usize,
		/// Maximum number of articles for each feed to print
		#[serde(default)]
		pub feed_limit: usize,
		/// Schedule for edition pressing automation
		pub schedule: String,
		/// Boolean to control if schedule is active
		pub schedule_enabled: bool,
		/// The vec containing each feed we will process
		pub feed: Vec<FeedEntry>,
	}

	/// Holds detailed information about a specific source RSS feed to
	/// pull articles from.  Configuration is held in the `[[feed]]` array in
	/// the configuration toml.
	///
	/// This is an example feed entry in the configuration that will pull a
	/// maximum of 10 articles no older than 3 days and place them into
	/// the "News" section.
	/// ```toml
	/// [[feed]]
	///   url = "https://yourfeedurl.com/rss.xml"
	///   feed_limit = 10
	///   max_age = 3
	///   section = "News"
	/// ```
	#[derive(Debug, Deserialize, Serialize, PartialEq)]
	pub struct FeedEntry {
		/// Feed URL
		pub url: String,
		/// Article count limit, default is all (0)
		#[serde(default)]
		pub feed_limit: usize,
		/// Article section, default is seen in the fn [default_section]
		#[serde(default = "default_section")]
		pub section: String,
		/// Max age, in days, before an article is skipped or ignored
		#[serde(default)]
		pub max_age: usize,
	}

	/// Contains the default section used when one is not provided within
	/// a feed's configuration.
	pub fn default_section() -> String {
		"Personal".to_string()
	}


}