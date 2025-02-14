use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::utils::command::BotCommands;    
use crate::{BotDialogue, Command, State};

pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

pub async fn handle_start(
    bot: Bot,
    msg: Message
) -> HandlerResult {
    bot.send_message(msg.chat.id, "👋  Hello!\n🤖  This bot was created to help\n        you find out the entropy\n        of your password.\n⌨️  Type /entropy or use\n        menu button to start").await?;

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
    bot.send_message(msg.chat.id, "Cancelled").await?;
    
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
        "🤖  This bot was created to\n        help you find out the entropy\n        of your password\n\n❓  Password entropy is the measure\n        of password strength — how effective\n        the password is against hackers\n\n📜  This bot is open source.\n        You can read it by clicking\n        the button below the message"
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
                format!("Recomended entropy:\n70 bits\n\nYour password entropy: \n{:.2} bits", entropy)
            ).await?;
            dialogue.update(State::Start).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please enter your password").await?;
        }
    }

    Ok(())
}
