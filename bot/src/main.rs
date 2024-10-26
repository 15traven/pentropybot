use teloxide::prelude::*;
use teloxide::types::Me;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::macros::BotCommands;
use serde::{Serialize, Deserialize};

pub mod handlers;

use handlers::{HandlerResult, handle_start};

pub type BotDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub enum State {
    #[default]
    Start
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start
}

fn main() {
    println!("Hello");
}
