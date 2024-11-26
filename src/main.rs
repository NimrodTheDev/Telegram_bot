mod X_Auth;
mod Coin_Data;
mod Response;

use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup, Message, UpdateKind};
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
    // X_Auth::twitter().await;
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, update: Update| async move{
        println!("{:?}", update);
        match update.kind {
            UpdateKind::Message(msg) =>{
                if let Some(s) = msg.text() {
                    println!("Homepage");
                    if let Ok(command) = cmd::parse(s, "TheSeiNewbieBot") {
                        println!("Homepage2");
                        let data = Coin_Data::get_sei_info().await;
                        match data {
                            Ok(data )=>{
                                match command {
                                    cmd::aboutsei =>{
                                        bot.send_message(msg.chat.id, Response::about_sei(&data)).await;
                                    }
                                    cmd::community =>{
                                        bot.send_message(msg.chat.id, Response::community(&data)).await;
                                    }
                                    cmd::start =>{
                                        // let help_text = cmd::descriptions();
                                        let keyboard = InlineKeyboardMarkup::new(vec![
                                            vec![
                                                InlineKeyboardButton::callback("About sei", "/aboutsei"),
                                                InlineKeyboardButton::callback("Join" ,"/community"),
                                            ],
                                        ]);
                                        bot.send_message(msg.chat.id, Response::Help()).reply_markup(keyboard).await.unwrap();
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
            }
            UpdateKind::CallbackQuery(query)=>{
                println!("Works");
                if let Some(data) = query.clone().data {
                    handle_callback(bot.clone(), query.clone(), data).await?;
                }
            }
            _=>{
                println!("W");
            }
        };
        respond(())
    }).await;
    Ok(())
}


async fn handle_callback(bot: Bot, query: CallbackQuery, data: String) -> ResponseResult<()> {
    // Process the callback data
    println!("Works");
    match data.as_str() {
        "aboutsei" => {
            if let Some(chat_id) = query.message.as_ref().map(|m| m.chat().id) {
                bot.send_message(chat_id, "About Sei: Sei is a blockchain network designed for...").await?;
            }
        }
        "community" => {
            if let Some(chat_id) = query.message.as_ref().map(|m| m.chat().id) {
                bot.send_message(chat_id, "Join the Sei community here: https://sei.io/community").await?;
            }
        }
        _ => {
            if let Some(chat_id) = query.message.as_ref().map(|m| m.chat().id) {
                bot.send_message(chat_id, "Unknown command").await?;
            }
        }
    }

    // Answer the callback query to remove the "loading" animation in the Telegram client
    bot.answer_callback_query(query.id).await?;
    Ok(())
}