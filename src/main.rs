use teloxide::{prelude::*, requests::ResponseResult, types::Message, utils::command, Bot};
use crate::command::BotCommands;
#[derive(BotCommands)]
#[command(rename="lowercase")]
enum cmd{
    AboutSei,
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    teloxide::repl(bot, |bot: Bot, msg: Message| async move{
        if let Some(s) = msg.text() {
            if let Ok(command) = cmd::parse(s, "SeiBot") {
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