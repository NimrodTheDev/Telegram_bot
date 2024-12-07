mod X_Auth;
mod Coin_Data;
mod Response;

use reqwest::Url;
use serde::de::value;
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
    #[command(description = "Guides users through wallet creation, sending/receiving sei and exporting private key... Coming soon.")]
    guide,
    wallet_guide,
    buy_sell_guide,
    private_key_guide,
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
    let url = Url::parse("https://telegram-bot-4ngf.onrender.com").unwrap();
    bot.set_webhook(url).await?;
    teloxide::repl(bot, |bot: Bot, msg: Message| async move{
        // println!("{:?}", update);
        if let Some(s) = msg.text() {
            if let Ok(command) = cmd::parse(s, "TheSeiNewbieBot") {
                println!("Homepage2");
                println!("Homepage");
                handle_command(bot, msg.clone(), command).await;
            }else{
                let value: Vec<&str> = s.split_whitespace().collect();
                let max_len = value.len();
                let last_value = value[max_len - 1];    
                if let Ok(command) = cmd::parse(last_value, "TheSeiNewbieBot"){
                    handle_command(bot, msg.clone(), command).await;
                };
        }}
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

async fn handle_command(bot: Bot, msg: Message, command: cmd){
        match command {
            cmd::aboutsei =>{
                bot.send_message(msg.chat.id, Response::about_sei().await).await;
            }
            cmd::community =>{
                bot.send_message(msg.chat.id, Response::community().await).await;
            }
            cmd::start =>{
                // let help_text = cmd::descriptions();
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::switch_inline_query_current_chat("About sei","/aboutsei"),
                        InlineKeyboardButton::switch_inline_query_current_chat("Join sei communtiy" ,"/community"),
                        InlineKeyboardButton::switch_inline_query_current_chat("Join sei communtiy" ,"/guide"),
                    ],
                ]);
                bot.send_message(msg.chat.id, Response::Help()).reply_markup(keyboard).await.unwrap();
            }
            cmd::guide=>{
                let keyboard = InlineKeyboardMarkup::new(vec![
                    vec![
                        InlineKeyboardButton::switch_inline_query_current_chat("Wallet creation","/wallet_guide"),
                        InlineKeyboardButton::switch_inline_query_current_chat("Send & Receive SEI" ,"/buy_sell_guide"),
                    ],
                    vec![
                        InlineKeyboardButton::switch_inline_query_current_chat("Export Private Key" ,"/private_key_guide"),
                    ]
                ]);
                bot.send_message(msg.chat.id, Response::Guide()).reply_markup(keyboard).await.unwrap();
            }
            _=>{
                bot.send_message(msg.chat.id, "Still working on the command Dev need rest ejor.").await;
            }
        }
}