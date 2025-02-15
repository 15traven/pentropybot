use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;    
use crate::{keyboards, messages, BotDialogue, Command, State};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start(
    bot: Bot,
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, messages::START_MESSAGE)
        .reply_markup(keyboards::menu_keyboard())
        .await?;

    Ok(())
}

pub async fn handle_help(
    bot: Bot,
    msg: Message
) -> HandlerResult {
    bot.send_message(
        msg.chat.id, 
        Command::descriptions().to_string()
    ).await?;

    Ok(())
}

pub async fn handle_cancel(
    bot: Bot,
    msg: Message,
    dialogue: BotDialogue
) -> HandlerResult {
    dialogue.update(State::Start).await?;
    bot.send_message(msg.chat.id, messages::CANCELLED_MESSAGE).await?;
    
    Ok(())
}

pub async fn handle_info(
    bot: Bot,
    msg: Message
) -> HandlerResult {
    let url = url::Url::parse("https://github.com/15traven/pentropybot")?;
    let keyboard = InlineKeyboardMarkup::new([[
        InlineKeyboardButton::url("Source code", url)
    ]]);
    bot.send_message(
        msg.chat.id,
        messages::INFO_MESSAGE
    ).reply_markup(keyboard).await?;

    Ok(())
}

pub async fn handle_entropy(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, messages::REQUEST_PASSWORD).await?;
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
            let wait_msg = bot.send_message(msg.chat.id, messages::WAIT_MESSAGE).await?;

            let entropy = password::calculate_entropy(password);
            let is_common = password::is_common(password);

            let _ = bot.delete_message(msg.chat.id, wait_msg.id).await?;
            bot.send_message(
                msg.chat.id, 
                format!("Recomended entropy:\n70 bits\n\nYour password entropy: \n{:.2} bits", entropy)
            ).await?;

            if is_common {
                bot.send_message(msg.chat.id, messages::COMMON_PASSWORD_MESSAGE).await?;
            }

            dialogue.update(State::Start).await?;
        }
        None => {
            bot.send_message(msg.chat.id, messages::REQUEST_PASSWORD).await?;
        }
    }

    Ok(())
}
