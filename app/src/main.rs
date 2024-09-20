use std::error::Error;
use std::io::Write;
use std::str::FromStr;
use hayagriva::io::to_yaml_str;
use hayagriva::types::EntryType;
use hayagriva::types::FormatString;
use hayagriva::types::QualifiedUrl;
use hayagriva::Entry;
use hayagriva::Library;
use rss::Channel;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde::Serialize;
use chrono::prelude::*;
use chrono::TimeDelta;

#[derive(Debug, Deserialize)]
struct FeedConfig {
    show_errors: bool, 
    #[serde(default)]
    feed_limit: usize,
    #[serde(default)]
    max_age: usize,
    feed: Vec<FeedEntry>,
}

#[derive(Debug, Deserialize)]
struct FeedEntry {
    url: String,
    #[serde(default)]
    feed_limit: usize,
    #[serde(default = "default_section")]
    section: String,
    #[serde(default)]
    max_age: usize,
}
fn default_section() -> String {
    "News".to_string()
}

#[derive(Debug, Serialize)]
struct Press {
    content: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    section: String,
    source: String,
    link: String,
    pub_date: String,
    title: String,
    bib_key: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct BiblioEntry {
    r#type: EntryType,
    key: String,
    title: String,
    date: String,
    url: String,
}

#[tokio::main]
async fn main() {
    println!("feedpress - pressing all the news that's fit to press");

    let mut file = File::open("../data/config.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");
   
    // Parse the config into a toml object
    let config: FeedConfig = match toml::from_str(&contents) {
        Ok(f) => f,
        Err(_) => {
            println!("Unable to parse feed entries from config toml.  Going to panic now.");
            panic!();
        },
    };

    let local_time: DateTime<Local> = Local::now();
    println!("Done parsing configuration.  Current time is: {}", local_time);

    // This is a placeholder for our pressed together content and related biblios
    let mut press_content: Vec<Content> = Vec::new();
    let mut press_biblio: Vec<BiblioEntry> = Vec::new();

    // For all of the feeds in our config... do stuff.
    let mut r: usize = 0;
    for this_entry in &config.feed {
        // println!("{:?}", this_entry.url);
        
        let channel: Channel = match get_feed(&this_entry.url).await {
            Ok(c) => c,
            Err(e) => {
                if config.show_errors {
                    println!("Unable to get feed URL: {} error={}",&this_entry.url, e);
                }
                continue
            },
        };

        // The section for this entry
        let entry_section = this_entry.section.to_string();
        
        // Use the feed limit, too
        let mut feed_limit: usize = config.feed_limit;
        if this_entry.feed_limit > 0 {
            feed_limit = this_entry.feed_limit;
        }

        let mut max_age: usize = config.max_age;
        if this_entry.max_age > 0 {
            max_age = this_entry.max_age;
        }

        println!("Processing: {} with feed limit of {} and max age of {} days", channel.title(), feed_limit, max_age );
        let mut i: usize = 0;

        for this_item in channel.items() {
            r = r+1;
            // If we have a feed limit, make sure we apply it.
            i = i+1;
            if feed_limit > 0 && i > feed_limit {
                break;
            }

            let pub_date = DateTime::parse_from_rfc2822(this_item.pub_date().unwrap()).unwrap();
            let article_age = local_time.fixed_offset() - pub_date;

            if article_age > TimeDelta::days(max_age as i64) {
                if config.show_errors {
                    println!("Article is {} days old, skipping.", article_age.num_days());
                }
                continue;
            }

            // Build a new struct of this particular content for outbound formatting
            let this_content = Content {
                section: entry_section.clone(),
                source: channel.description.to_string(),
                link: this_item.link().unwrap().to_string(),
                pub_date: this_item.pub_date().unwrap().to_string(),
                title: this_item.title().unwrap_or("No Title").to_string(),
                bib_key: format!("key-{}",r),
                content: this_item.description().unwrap().to_string(),
            };

            // Build also its related biblio entry
            let this_biblio = BiblioEntry {
                r#type: EntryType::Web,
                key: format!("key-{}",r),
                title: this_item.title().unwrap_or("No Title").to_string(),
                date: this_item.pub_date().unwrap().to_string(),
                url: this_item.link().unwrap().to_string(),
            };

            // Slap it into the outbound vector
            press_content.push(this_content);
            press_biblio.push(this_biblio);
        }
    }

    // This structure will likely be expanded, but for now contains the array of our
    // outbound, pressed, content.
    let this_press: Press = Press {
        content: press_content,
    };

    // Now, press_content contains our combined feed information and fields that are
    // readable by the typst templates.  Let's serialize them to a file.
    let toml = toml::to_string(&this_press).unwrap();

    // Write this goodness out to a file and its related -bib.yml version
    let mut file = File::create("../input/input.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

  

    // Create a biblio library for this edition
    let mut library = Library::new();
    for bib_entry in press_biblio {
        let mut entry: Entry = Entry::new(&bib_entry.key, bib_entry.r#type);
        entry.set_title(FormatString::from_str(&bib_entry.title).unwrap());
        entry.set_url(QualifiedUrl::from_str(&bib_entry.url).unwrap());
        library.push(&entry);
    }

    let yaml = to_yaml_str(&library).unwrap();
    let mut bib_file = File::create("../input/input-bib.yml").unwrap();
    bib_file.write_all(yaml.as_bytes()).unwrap();
    

}


async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}