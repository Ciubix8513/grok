use std::{collections::HashMap, time::Duration};

use rand::Rng;

use crate::mastodon_client::{Client, NotificationType, Post};

pub mod mastodon_client;
pub mod miskey_client;

fn generate_meow() -> String {
    let mut rng = rand::rng();
    let num_meows = rng.random_range(1..10);
    let meows = if rng.random_range(1..100) == 42 {
        include_str!(".././fox_noises")
    } else {
        include_str!(".././meows")
    }
    .lines()
    .collect::<Vec<_>>();

    let mut o = "".to_string();

    for _ in 0..num_meows {
        o += meows[rng.random_range(0..meows.len())];
        o += " ";
    }

    o.trim_end().into()
}

#[tokio::main]
async fn main() {
    let env = dotenv::vars().collect::<HashMap<_, _>>();
    let polling_period = env.get("POLLING_INTERVAL").unwrap().parse().unwrap();
    let token = env.get("MISKEY_TOKEN").unwrap().clone();
    let instance = env.get("INSTANCE").unwrap().clone();

    let miskey_client = miskey_client::Client::new(token.clone(), instance.clone());
    let masto_client = Client::new(token, instance);

    //Test the clients
    let _ = masto_client.me().await.unwrap();
    let _ = miskey_client.me().await.unwrap();

    loop {
        //Get notifications
        let notifications = masto_client.get_notifications().await;
        //If got some notifications immediately flush them
        if !notifications.is_empty() {
            miskey_client.flush_notifications().await.unwrap();
        }

        let names = notifications
            .into_iter()
            .filter(|i| i.r#type == NotificationType::mention)
            .map(|i| {
                let status = i.status.unwrap();
                (i.account.acct, status.id, status.visibility)
            })
            .collect::<Vec<_>>();

        for (username, status_id, visibility) in names {
            let meow = generate_meow();
            let post = Post {
                status: format!("@{username} {meow}"),
                in_reply_to_id: Some(status_id),
                visibility: Some(visibility),
                ..Default::default()
            };

            masto_client.create_post(post).await.unwrap();
        }

        tokio::time::sleep(Duration::from_secs(polling_period)).await
    }
}
