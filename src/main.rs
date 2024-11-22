use std::fmt::Write;

use teloxide::prelude::*;
use teloxide::types::Message;
use teloxide::utils::command::BotCommands;
use teloxide::Bot;

//TELOXIDE SHOULD BE A MACRO

#[derive(BotCommands)]
enum cmd{
    #[command(description = "Show a list of all commands.")]
    start,
    #[command(description = "Learn about sei and it's features.")]
    aboutsei,
    #[command(description = "Join and collaborate with the Sei community.")]
    community,
    #[command(description = "Guides users through waller creation, sending/receiving sei and exporting private key... Coming soon.")]
    guide,
    #[command(description = "Guide on how to buy sei with SimpleSwap.io... Coming soon.")]
    buysei,
    #[command(description = "Guide on how to export wallet keys and provide download link... Coming soon.")]
    exportwallet,
    #[command(description = "Provides a list of games on the sei network... Coming soon.")]
    games,
    #[command(description = "Swap tokens on dragon swap... Coming soon.")]
    swap,
    #[command(description = "A chatbot to answer all sei based question... Coming soon.")]
    chatbot,
}

#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    // get_sei_info().await;
    let bot = Bot::from_env();
    teloxide::repl(bot, |msg: Message, bot: Bot| async move{
        if let Some(s) = msg.text() {
            println!("Homepage");
            if let Ok(command) = cmd::parse(s, "TheSeiNewbieBot") {
                println!("Homepage2");
                let data = get_sei_info().await;
                match data {
                    Ok(data )=>{
                        match command {
                            cmd::aboutsei =>{
                                bot.send_message(msg.chat.id, about_sei(&data)).await;
                            }
                            cmd::community =>{
                                bot.send_message(msg.chat.id, community(&data)).await;
                            }
                            cmd::start =>{
                                let help_text = cmd::descriptions();
                                bot.send_message(msg.chat.id, help_text.to_string()).await;
                            }
                            _=>{
                                bot.send_message(msg.chat.id, "Still working on the command Dev need rest ejor.").await;
                            }
                        }
                    }
                    Err(err)=>{
                        println!("{err}");
                        bot.send_message(msg.chat.id, "Error fetching data").await;
                    }
                }
            };
        }
        respond(())
    }).await;
    Ok(())
}

use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoinGeckoResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub description: Description,
    pub links: Links,
    pub market_data: MarketData,
}

#[derive(Deserialize, Debug)]
pub struct Description {
    pub en: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Links {
    pub homepage: Vec<String>,
    pub twitter_screen_name: Option<String>,
    pub subreddit_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct MarketData {
    pub current_price: Prices,
    pub market_cap: Prices,
    pub total_volume: Prices,
    pub price_change_percentage_24h: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Prices {
    #[serde(rename = "usd")]
    pub usd: Option<f64>,
    #[serde(rename = "eur")]
    pub eur: Option<f64>,
    #[serde(rename = "btc")]
    pub btc: Option<f64>,
}

async fn get_sei_info() -> Result<CoinGeckoResponse, Error> {
    // let (id, _symbol, _name) = get_coin_data("sei-network".to_string()).await?;
    let url = format!("https://api.coingecko.com/api/v3/coins/{}","sei-network");
    let response: CoinGeckoResponse = reqwest::get(url).await?.json::<CoinGeckoResponse>().await?;
    Ok(response)
}


use reqwest;

#[derive(Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
}

async fn get_coin_data(coin_substring: String) -> Result<(String, String, String), reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/coins/list";
    let coins: Vec<Coin> = reqwest::get(url).await?.json().await?;
    let mut result: (String, String, String) = (String::new(),String::new(),String::new());
    for coin in coins {
        if coin.id.to_lowercase().contains(&coin_substring) {
            result.0 = coin.id.clone();
            result.1 = coin.symbol.clone();
            result.2 = coin.name.clone();
            println!("ID: {}, Symbol: {}, Name: {}", coin.id, coin.symbol, coin.name);
        }
    }

    Ok(result)
}


pub fn about_sei(response: &CoinGeckoResponse) -> String {
    let mut about = String::new();

    writeln!(
        &mut about,
        "🌟 *About Sei Network ({}):*\n",
        response.symbol.to_uppercase()
    )
    .unwrap();

    // Description
    if let Some(description) = &response.description.en {
        writeln!(&mut about, "{}\n", description).unwrap();
    } else {
        writeln!(
            &mut about,
            "Sei Network is a cutting-edge blockchain platform optimized for trading."
        )
        .unwrap();
    }

    // Current price and market data
    if let Some(price) = response.market_data.current_price.usd {
        writeln!(&mut about, "💵 *Current Price (USD):* ${:.2}", price).unwrap();
    } else {
        writeln!(&mut about, "💵 *Current Price:* Not available").unwrap();
    }

    if let Some(change_24h) = response.market_data.price_change_percentage_24h {
        writeln!(
            &mut about,
            "📈 *24h Price Change:* {:.2}%",
            change_24h
        )
        .unwrap();
    }

    if let Some(market_cap) = response.market_data.market_cap.usd {
        writeln!(
            &mut about,
            "🏦 *Market Cap (USD):* ${:.2}",
            market_cap
        )
        .unwrap();
    }

    // Add closing statement
    writeln!(
        &mut about,
        "\nSei Network empowers developers and users with efficient and reliable blockchain technology. \
        It's a perfect choice for anyone interested in blockchain trading innovations."
    )
    .unwrap();

    about
}

pub fn community(response: &CoinGeckoResponse) -> String {
    let mut community_info = String::new();

    writeln!(
        &mut community_info,
        "🌐 *Join the Sei Community ({}):*\n",
        response.symbol.to_uppercase()
    )
    .unwrap();

    // Homepage
    writeln!(
        &mut community_info,
        "🔗 *Official Website:* {}\n",
        response
            .links
            .homepage
            .get(0)
            .cloned()
            .unwrap_or_else(|| "Not available".to_string())
    )
    .unwrap();

    // Twitter
    if let Some(twitter) = &response.links.twitter_screen_name {
        writeln!(
            &mut community_info,
            "🐦 *Twitter:* https://twitter.com/{}\n",
            twitter
        )
        .unwrap();
    }

    // Reddit
    if let Some(reddit) = &response.links.subreddit_url {
        writeln!(
            &mut community_info,
            "👥 *Reddit Community:* {}\n",
            reddit
        )
        .unwrap();
    }

    // Call to action
    writeln!(
        &mut community_info,
        "Sei is an open and collaborative network! Join us on our platforms to discuss, contribute, \
        and help shape the future of blockchain technology. Whether you're a developer, trader, or enthusiast, \
        there's a place for you here!"
    )
    .unwrap();

    community_info
}
