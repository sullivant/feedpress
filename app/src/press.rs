pub mod press {
	use hayagriva::types::Date;
	use hayagriva::types::EntryType;
	use serde::Serialize;

	/// A container to hold all of our compiled [ContentEntry] items.
	#[derive(Debug, Serialize)]
	pub struct Press {
		pub content: Vec<ContentEntry>,
	}

	/// A specific item of content used by the layout engine to create a section of
	/// news or content in the final PDF.  Think of this like an "article" of sorts.
	///
	/// An example piece of content is as follows:
	/// ```toml
	///     [[content]]
	///     section = "News"
	///     source = "BBC News - World"
	///     link = "https://www.bbc.com/news/articles/<article code>"
	///     pub_date = "Fri, 20 Sep 2024 14:35:19 GMT"
	///     title = "The article title."
	///     bib_key = "key-5"
	///     content = `The entire content to be shown in the output pdf...`
	/// ```
	#[derive(Debug, Serialize)]
	pub struct ContentEntry {
		/// The section where this article appears on the PDF.  Its default is described
		/// within the function [default_section]
		pub section: String,
		/// The source of the article in its text form "NY Times" etc.
		pub source: String,
		/// The link to the direct article
		pub link: String,
		/// The publication date, gathered from the RSS feed
		pub pub_date: String,
		/// The title of the article as appears on the RSS feed
		pub title: String,
		/// The bibliography key, used to relate to an entry in [BiblioEntry]
		pub bib_key: String,
		/// The entire content of the article, as much as can be found in the RSS feed
		pub content: String,
	}

	/// The bibliographic information that links to a [ContentEntry] record and this
	/// conforms to the standard found in the [hayagriva] crate and project.
	#[derive(Debug, Serialize)]
	pub struct BiblioEntry {
		/// The default of [EntryType::Web]
		pub r#type: EntryType,
		/// The key that is linked via [ContentEntry]
		pub key: String,
		/// The title of the article
		pub title: String,
		/// The date published
		pub date: Date,
		/// The direct location of the article
		pub url: String,
	}

}