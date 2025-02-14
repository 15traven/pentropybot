use teloxide::prelude::*;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands;
use dptree::deps;
use serde::{Serialize, Deserialize};

pub mod handler_tree;
pub mod handlers;

use handler_tree::handler_tree;

pub type BotDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start,
    HandlePassword
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start
}

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported"
)]
pub enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "get more info about")]
    Info,
    #[command(description = "calculate password entropy")]
    Entropy,
    #[command(description = "cancel the current operation")]
    Cancel
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    Dispatcher::builder(bot, handler_tree())
        .dependencies(deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
}
