use teloxide::{prelude::*, requests::ResponseResult, types::Message, utils::command, Bot};
use crate::command::BotCommands;
#[derive(BotCommands)]
#[command(rename="lowercase")]
enum cmd{
    AboutSei,
}

#[tokio::main]
async fn main() {
    get_sei_info();
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move{
        if let Some(s) = msg.text() {
            if let Ok(command) = cmd::parse(s, "TheSeiNewbieBot") {
                handler(bot, msg, command);
            };
        }
        respond(())
    });
}

async fn handler(bot: Bot, message: Message, command: cmd)-> (){
    match command {
        cmd::AboutSei =>{
            bot.send_message(message.chat.id, "Works".to_string());
        }
        _=>{
            bot.send_message(message.chat.id, "All commands".to_string());
        }
    }
    ()
}

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct CoinGeckoResponse {
    description: Description,
    links: Links,
}

#[derive(Deserialize)]
struct Description {
    en: Option<String>,
}

#[derive(Deserialize)]
struct Links {
    homepage: Vec<String>,
    twitter_screen_name: Option<String>,
    subreddit_url: Option<String>,
}

async fn get_sei_info() -> Result<(), Error> {
    let url = "https://api.coingecko.com/api/v3/coins/sei";
    let response = reqwest::get(url).await?.json::<CoinGeckoResponse>().await?;
    // println!("response: {:?}", response.);
    // Extracting news-related data
    if let Some(description) = &response.description.en {
        println!("Sei Description: {}", description);
    }

    println!("Homepage: {:?}", response.links.homepage);
    if let Some(twitter) = &response.links.twitter_screen_name {
        println!("Twitter: https://twitter.com/{}", twitter);
    }
    if let Some(reddit) = &response.links.subreddit_url {
        println!("Reddit: {}", reddit);
    }

    Ok(())
}
