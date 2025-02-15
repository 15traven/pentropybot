use dptree::case;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateFilterExt, UpdateHandler};
use teloxide::dptree::filter;
use teloxide::prelude::*;
use crate::handlers::*;
use crate::keyboards;
use crate::{StartCommand, Command, State};

pub fn handler_tree() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .branch(case![StartCommand::Start].endpoint(handle_start))
        )
        .branch(
            case![State::Start].branch(
                Update::filter_message()
                    .branch(
                        filter(|msg: Message| msg.text() == Some(keyboards::ENTROPY_BUTTON))
                            .endpoint(handle_entropy)
                    )
                    .branch(
                        filter(|msg: Message| msg.text() == Some(keyboards::HELP_BUTTON))
                            .endpoint(handle_help)
                    )
                    .branch(
                        filter(|msg: Message| msg.text() == Some(keyboards::INFO_BUTTON))
                            .endpoint(handle_info)
                    )
            )
        )
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .branch(case![Command::Help].endpoint(handle_help))
                .branch(case![Command::Info].endpoint(handle_info))
                .branch(case![Command::Entropy].endpoint(handle_entropy))
                .branch(case![Command::Cancel].endpoint(handle_cancel))
        )
        .branch(
            Update::filter_message()
                .branch(case![State::HandlePassword].endpoint(handle_password))
        )
}
