use dptree::case;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateFilterExt, UpdateHandler};
use teloxide::prelude::*;
use crate::handlers::*;
use crate::{StartCommand, Command, State};

pub fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .branch(case![StartCommand::Start].endpoint(handle_start))
        )
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(handle_help))
                .branch(case![Command::Entropy].endpoint(handle_entropy))
        )
        .branch(
            Update::filter_message()
                .branch(case![State::HandlePassword].endpoint(handle_password))
        )
}
