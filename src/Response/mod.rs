use std::fmt::Write;
use crate::Coin_Data;

pub async fn about_sei() -> String {
    let mut about = String::new();
    let response = Coin_Data::get_sei_info().await;
    match response {
        Ok(response)=>{
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
        }
        Err(_)=>{
            writeln!(
                &mut about,"
                An error occurred  getting data about sei
                "
            ).unwrap();
        }
    }

    about
}

pub async fn community() -> String {
    let mut community_info = String::new();
    let response = Coin_Data::get_sei_info().await;
    match response {
        Ok(response)=>{
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
            writeln!(
                &mut community_info, "
communities 

                Block 24 
Block 24 is a community created with the primary goal of onboarding Nigerians into the world of crypto and SEI.
* Founded by active Sei community members Midas and Cryptostew. 
* Produces educational video content to enhance crypto understanding. 
* Hosts 'Block Talk,' a weekly X space discussing various topics. 
* Organizes IRL (In Real Life) events to engage and educate the community. 

X link: https://x.com/Block24_

discord link: https://discord.com/invite/PuT5Y4SqMx \n


                Crossover Studios 
A Web3 creative studio focusing on developing intellectual property through online content creation. 
* What began as a few lighthearted animated videos on Twitter has rapidly expanded, with intentions to collaborate with additional creators and establish a small community of creators.

X link: https://x.com/crossover_xo

Discord link: https://discord.gg/rXVgq6swQx \n

                Fuckers 
THE COMMUNITY’S COMMUNITY!!! 
* YRRRRR!!! 
* FUCK AROUND N FIND OUT 
* [REDACTED] OF [REDACTED] 

X link: https://x.com/fuckersforlife

Discord link: https://discord.com/invite/8PS9AaaqK3 \n


                S3 Labs 
First started as a community around those who enjoy Sei spaces. This is your hub to chat during spaces and ask questions of people on stage.

It has grown a lot since then and now are a place for people to network within the Sei ecosystem. S3 also incubates NFT projects, empowering artists and collectors, as well as helps dApps get connected with users and vise versa. 

X link: https://x.com/s3_labs?s=21&t=YJUDwtZOk1GgaJ7Bk7oAnA

Discord link: https://discord.com/invite/s3labs"
            ).unwrap();
            // Call to action
            writeln!(
                &mut community_info,
                "Sei is an open and collaborative network! Join us on our platforms to discuss, contribute, \
                and help shape the future of blockchain technology. Whether you're a developer, trader, or enthusiast, \
                there's a place for you here!"
            )
            .unwrap();
        }
        Err(_)=>{
            writeln!(
                &mut community_info,"
                An error occurred  getting data about sei
                "
            ).unwrap();
        }
    }

    community_info
}

pub fn Help()-> String{
    
    let mut Help = String::new();
    writeln!(&mut Help, "
    🌟 Welcome to @TheSeiNewbieBot! 🌟
Hi there! I’m here to guide you through everything Sei-related. Whether you’re a beginner or looking to dive deeper, I’ve got your back! 🚀

🔹 Getting Started
1️⃣ Follow us on Twitter for the latest updates and exclusive content 👉 https://x.com/TheSeiNewbiebot.
2️⃣ Use the buttons below to explore commands and interact with me. 😊

🤖 What Can I Do?
Tap the commands below to discover all the features and tools I offer! 🎉
    ").unwrap();
    Help
}