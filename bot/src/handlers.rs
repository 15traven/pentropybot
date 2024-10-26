use std::error::Error;
use teloxide::{prelude::*, types::Me};
use crate::{BotDialogue, StartCommand, State};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start(
    bot: Bot,
    msg: Message,
    me: Me
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Hello").await?;

    Ok(())
}

pub async fn handle_entropy(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please enter your password").await?;
    dialogue.update(State::HandlePassword).await?;

    Ok(())
}

pub async fn handle_password(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message
) -> HandlerResult {
    match msg.text() {
        Some(password) => {
            bot.send_message(msg.chat.id, password).await?;
            dialogue.update(State::Start).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter your password").await?;
        }
    }

    Ok(())
}
