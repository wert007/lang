use lang_core::vocab::Vocab;
use teloxide::{
    dispatching::{
        DpHandlerDescription,
        dialogue::{GetChatId, InMemStorage},
    },
    dptree::{deps, di::Injectable},
    prelude::*,
    types::InputPollOption,
};

const TOKEN: &'static str = include_str!("../token.txt").trim_ascii();

mod state;
use state::State;

type R = anyhow::Result<()>;
type Dg = Dialogue<State, InMemStorage<State>>;

#[derive(Clone)]
#[allow(unused)]
struct UpdateWithSuppliedChatId(Update, ChatId);

impl GetChatId for UpdateWithSuppliedChatId {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.1)
    }
}

#[tokio::main]
async fn main() -> R {
    let bot = Bot::new(TOKEN);

    let handler = dptree::entry()
        .map(|u: Update| {
            let i = u.chat_id().unwrap_or_else(|| match &u.kind {
                teloxide::types::UpdateKind::PollAnswer(poll_answer) => poll_answer
                    .voter
                    .chat()
                    .map(|c| c.id)
                    .or(poll_answer.voter.user().map(|u| ChatId::from(u.id)))
                    .unwrap(),
                err => todo!("{err:#?}"),
            });
            UpdateWithSuppliedChatId(u, i)
        })
        .enter_dialogue::<UpdateWithSuppliedChatId, InMemStorage<State>, State>()
        .branch(
            Update::filter_message()
                .branch(if_is_command("ask", ask))
                .endpoint(unhandled),
        )
        .branch(Update::filter_poll_answer().endpoint(ask))
        .branch(Update::filter_poll().endpoint(ask));

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
