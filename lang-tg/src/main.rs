use teloxide::{
    dispatching::{DpHandlerDescription, dialogue::InMemStorage},
    dptree::{deps, di::Injectable},
    prelude::*,
};

const TOKEN: &'static str = include_str!("../token.txt").trim_ascii();

mod state;
use state::State;

type R = anyhow::Result<()>;
type Dg = Dialogue<State, InMemStorage<State>>;

#[tokio::main]
async fn main() -> R {
    let bot = Bot::new(TOKEN);

    let handler = Update::filter_message()
        .enter_dialogue::<Update, InMemStorage<State>, State>()
        .branch(if_is_command("ask", ask))
        .endpoint(unhandled);

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .dependencies(deps![InMemStorage::<State>::new()])
        .build()
        .dispatch()
        .await;
    Ok(())
}

async fn ask(bot: Bot, message: Message, d: Dg) -> R {
    d.get_or_default().await?.init(&message);
    bot.send_message(message.chat.id, "Nice of you to ask!")
        .await?;
    Ok(())
}

async fn unhandled(update: Update) -> R {
    dbg!(update);
    Ok(())
}

fn is_command<'a>(cmd: &'static str) -> Handler<'a, R, DpHandlerDescription> {
    dptree::filter(move |m: Message| {
        m.text()
            .is_some_and(|t| &t.trim()[..1] == "/" && &t.trim()[1..] == cmd)
    })
}

fn if_is_command<'a, FnArgs, F: Injectable<R, FnArgs> + Send + Sync + 'a>(
    cmd: &'static str,
    handler: F,
) -> Handler<'a, R, DpHandlerDescription> {
    is_command(cmd).endpoint(handler)
}
