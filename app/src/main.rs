#![warn(missing_docs)]
#![allow(rustdoc::bare_urls)]
//! feedpress
//!
//! Pressing together all your RSS news thats fit to press.
//! - Supports rss feed standards
//! - Creates [typst](https://typst.app) output files.
//!
//!

use article_scraper::Readability;
use chrono::prelude::*;
use chrono::TimeDelta;
use clap::Parser;
use config::config::default_section;
use config::config::FeedConfig;
use config::config::FeedEntry;
use endpoints::endpoints::api_get_config;
use endpoints::endpoints::api_get_edition_list;
use endpoints::endpoints::api_press_edition;
use endpoints::endpoints::api_update_config;
use hayagriva::io::to_yaml_str;
use hayagriva::types::Date;
use hayagriva::types::EntryType;
use hayagriva::types::FormatString;
use hayagriva::types::QualifiedUrl;
use hayagriva::Entry;
use hayagriva::Library;
use html2md::parse_html_custom;
use press::press::BiblioEntry;
use press::press::ContentEntry;
use press::press::Press;
use rocket::fs::FileServer;
use rocket::Config;
use rss::Channel;
use spider_transformations::transformation::content::IgnoreTagFactory;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;
use url::Url;


mod endpoints;
mod editions;
mod config;
mod press;


#[macro_use] extern crate rocket;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the url to add to, or remove from, the feed collection
    #[arg(short, long, required(false), default_value_t = format!(""))]
    url: String,

    /// Will add the provided URL to the feed listing with default options
    #[arg(short, long, default_value_t = false)]
    add: bool,

    /// Will remove the provided URL from the feed listing
    #[arg(short, long, default_value_t = false)]
    remove: bool,

    /// Will start a configuration and status server on port 8081
    #[arg(short, long, default_value_t = false)]
    serve: bool,
}

/// Main application entrypoint
///
/// Does the thing it says on the tin, I suppose.  Gathers configuration data, processes each
/// feed, and creates content as well as biblio entries.
#[tokio::main]
async fn main() {
    println!("feedpress - pressing all the news that's fit to press");

    let rocket_config = Config {
        port: 8081,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::debug_default()
    };

    let args = Args::parse();

    if args.add {
        add_feed_url(&args.url);
        return;
    }
    if args.remove {
        remove_feed_url(&args.url);
        return;
    }
    if args.serve {
        println!("Staring feedpress server...");
        let _ = rocket::custom(&rocket_config)
        .mount("/api", rocket::routes![api_get_config])
        .mount("/api", rocket::routes![api_update_config])
        .mount("/api", rocket::routes![api_get_edition_list])
        .mount("/api", rocket::routes![api_press_edition])
        // .mount("/api", rocket::routes![api_get_edition])
        .mount("/editions", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../output")).rank(1))
        .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/static")))
        .launch()
        .await;

        return;
    }


    // If we are here, we are not doing any one off commands, so let's press the feeds into
    // a PDF and then exit.
    press_feeds().await;


}

/// Adds a provided feed URL to the array located in the configuration toml
fn add_feed_url(this_url: &str) {
    if this_url.is_empty() || !this_url.is_ascii() {
        println!("Invalid URL, doing nothing.");
        return;
    }
    println!("Adding URL to feed list: {}", this_url);

    let this_entry = FeedEntry {
        url: this_url.to_string(),
        feed_limit: 3,
        section: default_section(),
        max_age: 3
    };

    let mut config = get_config().unwrap();
    if !config.feed.contains(&this_entry) {
        config.feed.push(this_entry);
    }

    let toml = toml::to_string(&config).unwrap();

    // Write this updated config to a file
    let mut file = File::create("../data/config.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();


    return;
}

/// Removes a provided feed URL from the array located in the configuration toml
fn remove_feed_url(url: &str) {
    if url.is_empty() || !url.is_ascii() {
        println!("Invalid URL, doing nothing.");
        return;
    }
    println!("Removing URL {}", url);
}

/// Processes RSS feeds located in the configuration toml and then prepares an
/// output file able to be used by typst.
async fn press_feeds() {

    // Parse the config into a toml object
    let config: FeedConfig = get_config().unwrap();

    let local_time: DateTime<Local> = Local::now();
    println!(
        "Done parsing configuration.  Current time is: {}",
        local_time
    );

    // This is a placeholder for our pressed together content and related biblios
    let mut press_content: Vec<ContentEntry> = Vec::new();
    let mut press_biblio: Vec<BiblioEntry> = Vec::new();

    // For all of the feeds in our config... do stuff.
    let mut r: usize = 0; // This is our "key" for the biblio.
    for this_entry in &config.feed {
        let channel: Channel = match get_feed(&this_entry.url).await {
            Ok(c) => c,
            Err(e) => {
                if config.show_errors {
                    println!("Unable to get feed URL: {} error={}", &this_entry.url, e);
                }
                continue;
            }
        };

        // The section for this entry
        let entry_section = this_entry.section.to_string();

        // Use the feed limit, too
        let mut feed_limit: usize = config.feed_limit;
        if this_entry.feed_limit > 0 {
            feed_limit = this_entry.feed_limit;
        }

        // And the maximum age, in days
        let mut max_age: usize = config.max_age;
        if this_entry.max_age > 0 {
            max_age = this_entry.max_age;
        }

        println!(
            "Processing: {} with feed limit of {} and max age of {} days",
            channel.title(),
            feed_limit,
            max_age
        );
        let mut i: usize = 0;

        // For each item in this channel's current feed data, grab stuff and deal with it.
        for this_item in channel.items() {
            r += 1; // Increment our biblio key.

            // If we have a feed limit, make sure we apply it.
            i += 1;
            if feed_limit > 0 && i > feed_limit {
                break;
            }

            // TODO: A better way to deal with the dates.
            let pub_date = DateTime::parse_from_rfc2822(this_item.pub_date().unwrap()).unwrap();
            let mut bib_date = Date::from_year(pub_date.year());
            bib_date.day = Some(pub_date.day().try_into().unwrap());
            bib_date.month = Some(pub_date.month0().try_into().unwrap());

            let article_age = local_time.fixed_offset() - pub_date;

            if article_age > TimeDelta::days(max_age as i64) {
                if config.show_errors {
                    println!("Article is {} days old, skipping.", article_age.num_days());
                }
                continue;
            }

            // Article's link
            let article_link = this_item.link().unwrap().to_string();
            let article_short_content = this_item.description().unwrap_or("No Content").to_string();
            let article_content = scrape_this(&article_link)
                .await
                .unwrap_or(article_short_content.clone());

            // println!("{}", article_content);

            // Build a new struct of this particular content for outbound formatting
            let this_content = ContentEntry {
                section: entry_section.clone(),
                source: channel.description.to_string(),
                link: article_link,
                pub_date: this_item.pub_date().unwrap().to_string(),
                title: this_item.title().unwrap_or("No Title").to_string(),
                bib_key: format!("key-{}", r),
                content: article_content,
            };

            // Build also its related biblio entry
            let this_biblio = BiblioEntry {
                r#type: EntryType::Web,
                key: format!("key-{}", r),
                title: this_item.title().unwrap_or("No Title").to_string(),
                date: bib_date,
                url: this_item.link().unwrap().to_string(),
            };

            // Slap it into the outbound vectors
            press_content.push(this_content);
            press_biblio.push(this_biblio);
        }
    }

    // Call out and create our content and biblio files.
    process_content(press_content);
    process_biblio(press_biblio);
}

/// Does what it says on the tin.
fn get_config() -> Result<FeedConfig, String> {
    let mut file = File::open("../data/config.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    // Parse the config into a toml object
    let config: FeedConfig = match toml::from_str(&contents) {
        Ok(f) => f,
        Err(_) => {
            println!("Unable to parse feed entries from config toml.  Going to panic now.");
            panic!();
        }
    };

    Ok(config)
}

/// Utilizes [Readability] to scrape the article's provided link and then send it through
/// a simple html -> markdown processor.
async fn scrape_this(article_link: &String) -> Option<String> {
    let html = reqwest::get(article_link)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let base_url = Url::parse("https://feedpress.dev/").unwrap();
    let extracted_html = Readability::extract(&html, Some(base_url)).await;

    // println!("{:?}", extracted_html);

    // Parse the extracted HTML into markdown, ignoring some tags
    let mut tag_factory: HashMap<String, Box<dyn html2md::TagHandlerFactory>> = HashMap::new();
    let tag = Box::new(IgnoreTagFactory {});
    tag_factory.insert(String::from("script"), tag.clone());
    tag_factory.insert(String::from("a"), tag.clone());
    tag_factory.insert(String::from("img"), tag.clone());
    tag_factory.insert(String::from("i"), tag.clone());

    let md = parse_html_custom(
        &extracted_html.unwrap_or("**NO CONTENT**".to_string()),
        &tag_factory,
        true,
    );

    Some(md)
}

/// Creates the output file used in the typsetting portion of this process.
fn process_content(press_content: Vec<ContentEntry>) -> bool {
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

    true
}

/// Creates the bibliographic entry corresponding to each of the content entries.
fn process_biblio(press_biblio: Vec<BiblioEntry>) -> bool {
    // Create a biblio library for this edition and add entries for each of the keys
    let mut library = Library::new();
    for bib_entry in press_biblio {
        let mut entry: Entry = Entry::new(&bib_entry.key, bib_entry.r#type);
        entry.set_title(FormatString::from_str(&bib_entry.title).unwrap());
        entry.set_url(QualifiedUrl::from_str(&bib_entry.url).unwrap());
        entry.set_date(bib_entry.date);
        library.push(&entry);
    }

    let yaml = to_yaml_str(&library).unwrap();
    let mut bib_file = File::create("../input/input-bib.yml").unwrap();
    bib_file.write_all(yaml.as_bytes()).unwrap();

    true
}

/// Gets the feed data in the form of a [Channel]
async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
