use std::time::{UNIX_EPOCH, Duration};
use std::env;
use structopt::StructOpt;
use reqwest::Error;
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::prelude::DateTime;
use chrono::Utc;

const DISCORD_EPOCH: u64 = 1420070400000;

#[derive(StructOpt)]
struct Cli {
    id: i64,
}

#[derive(Deserialize)]
struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().expect("Failed to load .env file");
    let args = Cli::from_args();

    let bin_id = format!("{:b}", args.id);
    let bin_id_2 = format!("{:0>64}", bin_id);
    let bin_time = &bin_id_2[..42];
    let time = u64::from_str_radix(bin_time, 2).unwrap();

    let d = UNIX_EPOCH + Duration::from_millis(time+DISCORD_EPOCH);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%a, %Y-%m-%d %H:%M:%S").to_string();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let url = format!("https://discord.com/api/v9/users/{}", args.id);
    let client = reqwest::Client::new();
    let resp = client
    .get(url)
    .header(AUTHORIZATION, "Bot ".to_string() + &token)
    .send()
    .await?;

    if resp.status().as_str() == "200" {
        let json_resp = resp.json::<User>().await?;

        println!("User Id: {}", json_resp.id);
        println!("Username: {}", json_resp.username);
        println!("Discriminator: {}", json_resp.discriminator);
        println!("Avatar URL: https://cdn.discordapp.com/avatars/{}/{}.webp?size=256", json_resp.id,json_resp.avatar);
        println!("Created At: {}", timestamp_str);

    } else if resp.status().as_str() == "401"{
        println!("401 Unauthorized");
        println!("Your token might be invalid, check if your token in /.env is properly written.");
    } else if resp.status().as_str() == "404" {
        println!("404 Not found");
        println!("Your ID does not belong to a user or does not exist.");
    }
    
    Ok(())
}
