use std::error::Error;
use rss::Channel;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
struct FeedConfig {
    feed_limit: usize,
    feed: Vec<FeedEntry>,
}

#[derive(Debug, Deserialize)]
struct FeedEntry {
    url: String,
    #[serde(default)]
    feed_limit: usize,
}

#[derive(Debug, Serialize)]
struct Press {
    content: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    source: String,
    link: String,
    pub_date: String,
    title: String,
    bib_key: String,
    content: String,
}

#[tokio::main]
async fn main() {
    let mut file = File::open("../data/feeds.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
   
    // Parse the contents into a toml object
    let config: FeedConfig = toml::from_str(&contents).unwrap();
    // println!("{:?}", config);

    let mut press_content: Vec<Content> = Vec::new();

    for this_entry in &config.feed {
        // println!("{:?}", this_entry.url);
        let channel = get_feed(&this_entry.url).await.unwrap();
        
        let mut feed_limit: usize = config.feed_limit;
        feed_limit = this_entry.feed_limit;

        // Use the feed limit, too
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
                source: channel.description.to_string(),
                link: this_item.link().unwrap().to_string(),
                pub_date: this_item.pub_date().unwrap().to_string(),
                title: this_item.title().unwrap().to_string(),
                bib_key: "harry".to_string(),
                content: this_item.description().unwrap().to_string(),
            };

            // Slap it into the outbound vector
            press_content.push(this_content);
        }
    }

    let this_press: Press = Press {
        content: press_content,
    };

    // Now, press_content contains our combined feed information and fields that are
    // readable by the typst templates.  Let's serialize them to a file.
    let toml = toml::to_string(&this_press).unwrap();
    println!("{}", toml);

    // println!("{:?}", press_content);

}


async fn get_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}