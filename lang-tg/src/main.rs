use lang_core::vocab::Vocab;
use teloxide::{
    dispatching::dialogue::InMemStorage, dptree::deps, prelude::*, types::InputPollOption,
};

const TOKEN: &'static str = include_str!("../token.txt").trim_ascii();

mod state;
mod utils;
use state::State;
use utils::*;

#[tokio::main]
async fn main() -> R {
    let bot = Bot::new(TOKEN);

    let handler = dptree::entry()
        .map(UpdateWithSuppliedChatId::ensure_id)
        .enter_dialogue::<UpdateWithSuppliedChatId, InMemStorage<State>, State>()
        .branch(
            Update::filter_message()
                .branch(if_is_command("ask", ask))
                .endpoint(unhandled),
        )
        .branch(Update::filter_poll_answer().endpoint(ask));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
    Ok(())
}

async fn ask(bot: Bot, update: Update, d: Dg) -> R {
    let s = d.get_or_default().await?;
    s.init(&update);
    let v: Vocab = s.lang().vocab(0);
    bot.send_poll(
        d.chat_id(),
        format!("What does {} mean?", v.a()),
        [v.b(), "Wrong answer"].map(InputPollOption::new),
    )
    .correct_option_id(0)
    .is_anonymous(false)
    .type_(teloxide::types::PollType::Quiz)
    .await?;
    Ok(())
}
