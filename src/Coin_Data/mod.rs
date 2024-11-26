use reqwest;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct Coin {
    id: String,
    symbol: String,
    name: String,
}



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

pub async fn get_sei_info() -> Result<CoinGeckoResponse, Error> {
    // let (id, _symbol, _name) = get_coin_data("sei-network".to_string()).await?;
    let url = format!("https://api.coingecko.com/api/v3/coins/{}","sei-network");
    let response: CoinGeckoResponse = reqwest::get(url).await?.json::<CoinGeckoResponse>().await?;
    Ok(response)
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