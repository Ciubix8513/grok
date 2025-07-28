use std::time::Duration;

use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::mastodon_client::{Client, NotificationType, Post};

pub mod mastodon_client;
pub mod misskey_client;

fn generate_response(config: &Config, post_text: &Option<String>) -> (String, u32) {
    let mut rng = rand::rng();

    for (ind, r) in config.responses.iter().enumerate() {
        //First check if this is not the last dictionary, and if it is use it regardless of any
        //other checks
        if !(ind == config.responses.len() - 1) {
            //First try to get random
            if rng.random_range(0..100) > r.chance {
                //Randomness check did not succeed<F
                continue;
            }

            //Check if it matches the regex
            if let Some(regex) = &r.regex {
                if let Some(text) = post_text {
                    let regex = Regex::new(&regex).unwrap();

                    if !regex.is_match(text) {
                        continue;
                    }
                } else {
                    //There's no post text so it will never match the regex
                    continue;
                }
            }
        }

        let num_words = rng.random_range(r.min_words..r.max_words);

        let mut o = String::new();

        for _ in 0..num_words {
            o += &r.words[rng.random_range(0..r.words.len())];
            o += " ";
        }

        return (o.trim_end().into(), num_words);
    }

    unreachable!()
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    ///% chance that the bot will reply  with the following words
    chance: u32,
    ///Checks the post against this regex and uses this dictionary if it matches
    regex: Option<String>,
    ///Minimum number of words the bot will respond with
    min_words: u32,
    ///Maximum number of words the bot will respond with
    max_words: u32,
    ///Wether the dictionary contains any emoji
    contains_emoji: bool,
    ///Dictionary of words the bot will reply with
    ///
    ///Note:
    ///
    ///Order matters, the program will try to generate responses in order, so chance percentages
    ///are a bit skewed. For example if the first response has a chance of 10%, then 2nd has 15%
    ///and 3rd has 20%, the first one will have a chance of 10% then the 2nd one will have 15% of
    ///100 - 10%, and if both fail then the last one will be used.
    words: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    ///Instance URL
    instance: String,
    ///Api access token
    token: String,
    ///How much time to wait before checking notifications again
    polling_interval: u64,
    ///Things the bot can respond with
    responses: Vec<Response>,
}

fn generate_default_config() -> Config {
    Config {
        instance: "https://test.com".into(),
        token: "API_TOKEN".into(),
        polling_interval: 10,
        responses: vec![
            Response {
                regex: Some(r"(@.*)*is this true\?".into()),
                chance: 100,
                min_words: 1,
                max_words: 2,
                contains_emoji: true,
                words: vec![
                    "meow !!!".into(),
                    " ‌:neocat_sign_yes:".into(),
                    " ‌:neocat_sign_no:".into(),
                    "nyaa ?".into(),
                    " ‌:neocat_confused:".into(),
                    " ‌:neocat_glare:".into(),
                    " ‌:neocat_woozy:".into(),
                    " ‌:neocat_googly_woozy:".into(),
                ],
            },
            Response {
                regex: None,
                chance: 10,
                min_words: 1,
                max_words: 10,
                contains_emoji: true,
                words: vec![
                    "waf".into(),
                    "arrf".into(),
                    "awrfaf".into(),
                    ":neofox_floof:".into(),
                    ":fluffy_tail:".into(),
                ],
            },
            Response {
                regex: None,
                chance: 100,
                min_words: 1,
                max_words: 10,
                contains_emoji: false,
                words: vec![
                    "meow".into(),
                    "mew".into(),
                    "nyaaa".into(),
                    ":3".into(),
                    "mrrp".into(),
                    "mrmeow".into(),
                    ":neocat_floof:".into(),
                    ":neocat_flop:".into(),
                ],
            },
        ],
    }
}

#[tokio::main]
async fn main() {
    tokio::time::sleep(Duration::from_secs(10)).await;

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() > 1 {
        match args[1].as_str() {
            "-c" => {
                let c = generate_default_config();
                let s = toml::to_string(&c).unwrap();

                std::fs::write("./config.toml", s).unwrap();
                return;
            }
            _ => {
                println!("Usage: grok [-h|-c]");
                println!("\t-h displays this menu");
                println!("\t-c creates default config file");
                return;
            }
        }
    }

    let str = std::fs::read_to_string("./config.toml").unwrap();

    let config: Config = toml::from_str(&str).unwrap();
    println!("Parsed confg file");

    let misskey_client = misskey_client::Client::new(config.token.clone(), config.instance.clone());
    let masto_client = Client::new(config.token.clone(), config.instance.clone());

    //Test the clients
    let _ = masto_client.me().await.unwrap();
    let _ = misskey_client.me().await.unwrap();

    println!("Connected to misskey and mastodon");

    loop {
        println!("Checking notifications");
        //Get notifications
        let notifications = masto_client.get_notifications().await;
        //If got some notifications immediately flush them
        if !notifications.is_empty() {
            println!("Clearing notifications");
            misskey_client.flush_notifications().await.unwrap();
        }

        let names = notifications
            .into_iter()
            .filter(|i| i.r#type == NotificationType::mention)
            .map(|i| {
                let status = i.status.unwrap();
                (
                    i.account.acct,
                    status.text,
                    status.id,
                    status.visibility,
                    status.mentions,
                )
            })
            .collect::<Vec<_>>();

        println!("Replying");

        for (username, text, status_id, visibility, mentions) in names {
            let mut meow = generate_response(&config, &text);

            //i sure love sharkey
            loop {
                if meow.1 == 1 {
                    if meow.0.chars().next().unwrap() == ':'
                        && meow.0.chars().last().unwrap() == ':'
                    {
                        println!("regenerating the meow");
                        meow = generate_response(&config, &text);
                        continue;
                    }
                }
                break;
            }

            let mut pings = String::new();

            for i in mentions {
                //Probably don't need this
                if i.username == "grok" {
                    println!("Skipped self");
                    continue;
                }
                // https://lunar.place/@luna
                let binding = i.url.split("https://").collect::<Vec<_>>();

                //lunar.place/@luna
                let instance = binding.last().unwrap().split("/").next().unwrap();

                pings += &format!("@{}@{instance} ", i.username);
            }

            //Check if the meow is just a single emoji
            let post = Post {
                status: format!("@{username} {pings}{}", meow.0),
                in_reply_to_id: Some(status_id),
                visibility: Some(visibility),
                ..Default::default()
            };

            masto_client.create_post(post).await.unwrap();
        }

        tokio::time::sleep(Duration::from_secs(config.polling_interval)).await;
    }
}
