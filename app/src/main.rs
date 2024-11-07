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
use config::default_section;
use config::FeedConfig;
use config::FeedEntry;
use endpoints::*;
use hayagriva::io::to_yaml_str;
use hayagriva::types::EntryType;
use hayagriva::types::FormatString;
use hayagriva::types::QualifiedUrl;
use hayagriva::Entry;
use hayagriva::Library;
use html2md::parse_html_custom;
use press::*;
use reqwest::Client;
use rocket::fs::FileServer;
use rocket::Config;
use rss::Channel;
use spider_transformations::transformation::content::IgnoreTagFactory;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;
use std::time::Duration;
use url::Url;


use log::{error, info, warn};

mod endpoints;
mod editions;
mod config;
mod press;

#[macro_use] extern crate rocket;

/// The package version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Seconds allowed before timing out when requesting a URL
const REQUEST_TIMEOUT_SECS: u64 = 10;

/// Command line arguments meant to provide ability to add or remove feeds,
/// or press into an edition, via scheduled tasks, cronts, etc.
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
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("feedpress - pressing all the news that's fit to press");

    let rocket_config = Config {
        port: 8081,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        cli_colors: false,
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
        info!("Staring feedpress server...");
        let _ = rocket::custom(&rocket_config)
        .mount("/api", rocket::routes![api_get_config])
        .mount("/api", rocket::routes![api_update_config])
        .mount("/api", rocket::routes![api_get_edition_list])
        .mount("/api", rocket::routes![api_press_edition])
        .mount("/api", rocket::routes![api_remove_edition])
        .mount("/api", rocket::routes![api_get_version])
        .mount("/api", rocket::routes![api_get_logs])
        // .mount("/api", rocket::routes![api_get_edition])
        .mount("/editions", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../output")).rank(1))
        .mount("/", FileServer::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../assets/static")))
        .launch()
        .await;

        return;
    }

    // When doing nothing else, just create an edition allowing for CLI execution.
    create_edition().await;

}

/// Adds a provided feed URL to the array located in the configuration toml
fn add_feed_url(this_url: &str) {
    if this_url.is_empty() || !this_url.is_ascii() {
        warn!("Invalid URL, doing nothing.");
        return;
    }
    info!("Adding URL to feed list: {}", this_url);

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
}

/// Removes a provided feed URL from the array located in the configuration toml
fn remove_feed_url(url: &str) {
    if url.is_empty() || !url.is_ascii() {
        warn!("Invalid URL, doing nothing.");
        return;
    }
    info!("Removing URL {}", url);
}

/// Really a soft of collected convenience method that allows for an edition to be 
/// created both from the endpoint API and the command line.  Does all that is necessary.
async fn create_edition() {
    let local_time: DateTime<Local> = Local::now();
    let filename = format!("{}", local_time.format("%Y%m%d"));

    let output_png_path = format!("../output/{}.png", &filename);
    let output_pdf_path = format!("../output/{}.pdf", &filename);

    info!("Pressing new edition with filename: {}.", &output_pdf_path);

    // Press our feeds first to create a new input file.
    press_feeds().await;

    // Compile them into both PDF and PNG formats.
    compile_feeds(&output_pdf_path, &output_png_path).await;
}

/// Processes RSS feeds located in the configuration toml and then prepares an
/// output file able to be used by typst.
async fn press_feeds() {

    // Parse the config into a toml object
    let config: FeedConfig = get_config().unwrap();

    let local_time: DateTime<Local> = Local::now();
    info!(
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
                    error!("Unable to get feed URL: {} error={}", &this_entry.url, e);
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

        info!(
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

            let pub_date = DateTime::parse_from_rfc2822(this_item.pub_date().unwrap()).unwrap();
            if article_too_old(local_time, pub_date, max_age, config.show_errors) {
                continue;
            }

            // Article's link
            let article_link = this_item.link().unwrap().to_string();
            let article_short_content = this_item.description().unwrap_or("No Content").to_string();
            let article_content = scrape_this(&article_link)
                .await
                .unwrap_or(article_short_content.clone());

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

fn article_too_old(local_time: DateTime<Local>, pub_date: DateTime<FixedOffset>, max_age: usize, show_errors: bool) -> bool {
    let article_age = local_time.fixed_offset() - pub_date;
    if article_age > TimeDelta::days(max_age as i64) {
        if show_errors {
            warn!("Article is {} days old, skipping.", article_age.num_days());
        }
        return true;
    }
    false
}

/// Will compile the created feed data into both PDF and PNG formats (for a 1st page thumbnail)
async fn compile_feeds(output_pdf_path: &str, output_png_path: &str) {
    // Sample command: 
    // typst compile templates/feedpress.typ output/feedpress.pdf --root ./
    
    info!("Calling typst for compilation");

    let output = Command::new("typst")
    .arg("compile")
    .arg("../templates/feedpress.typ")
    .arg(output_pdf_path)
    .arg("--root")
    .arg("../")
    .output()
    .expect("Failed to execute command");

    if output.status.success() {
    	info!("Executed compile: {:?}", output.stdout);
    } else {
    	warn!("Trouble executing compile: {:?}", output.stderr);
    }

    info!("Executing pdf to png image generation for thumbnail.");

    // once an output file PDF is created use the utility pdftoppm to create a PNG
    // of the first page of the PDF, located in the sam eoutput directory.
    let output_png = Path::new(&output_png_path);
    let input_pdf = Path::new(&output_pdf_path);

    let png_output = Command::new("pdftoppm")
    .arg(input_pdf)
    .arg("-png")
    .arg("-singlefile")
    .arg("-f")
    .arg("1") // First page
    .arg("-l")
    .arg("1") // Only one page
    .arg("-r")
    .arg("100") // Resolution (DPI)
    .arg(output_png.with_extension(""))
    .output()
    .expect("Failed to execute pdftoppm command");

    // Check if the command succeeded
    if png_output.status.success() {
    	info!("PNG file created: {:?}", output_png);
    } else {
    	warn!(
    		"Error creating PNG: {}",
    		String::from_utf8_lossy(&output.stderr)
    	);
    }
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
            error!("Unable to parse feed entries from config toml.  Going to panic now.");
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
       
        library.push(&entry);
    }

    let yaml = to_yaml_str(&library).unwrap();
    let mut bib_file = File::create("../input/input-bib.yml").unwrap();
    bib_file.write_all(yaml.as_bytes()).unwrap();

    true
}

/// Gets the feed data in the form of a [Channel]
async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let client = Client::new();
    let timeout_duration = Duration::from_secs(REQUEST_TIMEOUT_SECS);

    let response = client.get(url)
        .timeout(timeout_duration)
        .send()
        .await;

    let content=  match response {
        Ok(res) => res.bytes().await?,
        Err(err) => {
            if err.is_timeout() {
                warn!("Request has timed out for url: {}",url);
            } else {
                warn!("Request has failed for url {} with reason: {:?}", url, err);
            }

            return Err(Box::new(err));
        }
    };

    // let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
