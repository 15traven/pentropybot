use std::error::Error;
use teloxide::{prelude::*, typse::Me};
use crate::{BotDialogue, StartCommand, State};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start(
    bot: Bot,
    msg: Message,
    me: Me
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Hello");
    
    Ok(())
}
