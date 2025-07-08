use std::{collections::HashMap, time::Duration};

use rand::Rng;

use crate::mastodon_client::{Client, NotificationType, Post};

pub mod mastodon_client;
pub mod miskey_client;

fn generate_meow() -> (String, u32) {
    let mut rng = rand::rng();
    let num_meows = rng.random_range(1..10);
    let meows = if rng.random_range(1..100) == 42 {
        include_str!(".././fox_noises")
    } else {
        include_str!(".././meows")
    }
    .lines()
    .collect::<Vec<_>>();

    let mut o = String::new();

    for _ in 0..num_meows {
        o += meows[rng.random_range(0..meows.len())];
        o += " ";
    }

    (o.trim_end().into(), num_meows)
}

#[tokio::main]
async fn main() {
    let env = dotenv::vars().collect::<HashMap<_, _>>();
    let polling_period = env.get("POLLING_INTERVAL").unwrap().parse().unwrap();
    let token = env.get("MISKEY_TOKEN").unwrap().clone();
    let instance = env.get("INSTANCE").unwrap().clone();
    println!("Got env vars");

    let miskey_client = miskey_client::Client::new(token.clone(), instance.clone());
    let masto_client = Client::new(token, instance);

    //Test the clients
    let _ = masto_client.me().await.unwrap();
    let _ = miskey_client.me().await.unwrap();

    println!("Connected to miskey and mastodon");

    loop {
        println!("Checking notifications");
        //Get notifications
        let notifications = masto_client.get_notifications().await;
        //If got some notifications immediately flush them
        if !notifications.is_empty() {
            println!("Clearing notifications");
            miskey_client.flush_notifications().await.unwrap();
        }

        let names = notifications
            .into_iter()
            .filter(|i| i.r#type == NotificationType::mention)
            .map(|i| {
                let status = i.status.unwrap();
                (
                    i.account.acct,
                    status.id,
                    status.visibility,
                    status.mentions,
                )
            })
            .collect::<Vec<_>>();

        println!("Replying");

        for (username, status_id, visibility, mentions) in names {
            let mut meow = generate_meow();

            //i sure love sharkey
            loop {
                if meow.1 == 1 {
                    if meow.0.chars().next().unwrap() == ':'
                        && meow.0.chars().last().unwrap() == ':'
                    {
                        println!("regenerating the meow");
                        meow = generate_meow();
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

        tokio::time::sleep(Duration::from_secs(polling_period)).await;
    }
}
