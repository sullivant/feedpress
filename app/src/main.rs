use std::error::Error;
use std::io::Write;
use rss::Channel;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct FeedConfig {
    #[serde(default)]
    feed_limit: usize,
    show_errors: bool, 
    feed: Vec<FeedEntry>,
}

#[derive(Debug, Deserialize)]
struct FeedEntry {
    url: String,
    #[serde(default)]
    feed_limit: usize,
    #[serde(default = "default_section")]
    section: String
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
struct Biblio {
    r#type: String,
    key: String,
    title: String,
    date: String,
    url: String,
}

#[tokio::main]
async fn main() {
    let mut file = File::open("../data/feeds.toml").expect("Failed to open file");
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
    // println!("{:?}", config);

    // This is a placeholder for our pressed together content and related biblios
    let mut press_content: Vec<Content> = Vec::new();
    let mut press_biblio: Vec<Biblio> = Vec::new();


    // For all of the feeds in our config... do stuff.
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

        println!("Processing: {} with feed limit of {}", channel.title(), feed_limit);
        let mut i = 0;

        for this_item in channel.items() {
            // If we have a feed limit, make sure we apply it.
            i = i+1;
            if feed_limit > 0 && i > feed_limit {
                break;
            }

            // Build a new struct of this particular content for outbound formatting
            let this_content = Content {
                section: entry_section.clone(),
                source: channel.description.to_string(),
                link: this_item.link().unwrap().to_string(),
                pub_date: this_item.pub_date().unwrap().to_string(),
                title: this_item.title().unwrap().to_string(),
                bib_key: "harry".to_string(),
                content: this_item.description().unwrap().to_string(),
            };

            // Build also its related biblio entry
            let this_biblio = Biblio {
                r#type: "Web".to_string(),
                key: format!("key-{}",i),
                title: this_item.title().unwrap().to_string(),
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
    let mut file = File::create("../input/20240919.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    // println!("{:?}",press_biblio);

}


async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}