use teloxide::prelude::*;
use teloxide::types::Me;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands;
use dptree::deps;
use serde::{Serialize, Deserialize};

pub mod handler_tree;
pub mod handlers;

use handler_tree::handler_tree;
use handlers::{HandlerResult, handle_start};

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
    #[command(description = "calculate password entropy")]
    Entropy
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
