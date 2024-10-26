use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, Me};
use teloxide::utils::command::BotCommands;    
use crate::{BotDialogue, StartCommand, Command, State};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start(
    bot: Bot,
    msg: Message,
    me: Me
) -> HandlerResult {
    bot.send_message(msg.chat.id, "Hello").await?;

    Ok(())
}

pub async fn handle_help(
    bot: Bot,
    msg: Message,
    me: Me
) -> HandlerResult {
    bot.send_message(
        msg.chat.id, 
        Command::descriptions().to_string()
    ).await?;

    Ok(())
}

pub async fn handle_info(
    bot: Bot,
    msg: Message,
    me: Me
) -> HandlerResult {
    let url = url::Url::parse("https://github.com/15traven/pentropybot")?;
    let keyboard = InlineKeyboardMarkup::new([[
        InlineKeyboardButton::url("Source code", url)
    ]]);
    bot.send_message(
        msg.chat.id,
        "This bot was created to help you find out the entropy of your password\n\nPassword entropy is the measure of password strength — how effective the password is against hackers\n\nThis bot is open source. You can read it by clicking the button below the message"
    ).reply_markup(keyboard).await?;

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
            let entropy = entropy::calculate_entropy(password);
            bot.send_message(
                msg.chat.id, 
                format!("Recomended entropy:\n70\nYour password entropy: \n{:.2}", entropy)
            ).await?;
            dialogue.update(State::Start).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter your password").await?;
        }
    }

    Ok(())
}
