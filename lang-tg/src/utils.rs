use teloxide::{
    dispatching::{
        DpHandlerDescription,
        dialogue::{GetChatId, InMemStorage},
    },
    dptree::di::Injectable,
    prelude::*,
};

use crate::state::State;

pub type R = anyhow::Result<()>;
pub type Dg = Dialogue<State, InMemStorage<State>>;

#[derive(Clone)]
#[allow(unused)]
pub struct UpdateWithSuppliedChatId(Update, ChatId);

impl UpdateWithSuppliedChatId {
    pub fn ensure_id(update: Update) -> Self {
        let id = update.chat_id().unwrap_or_else(|| match &update.kind {
            teloxide::types::UpdateKind::PollAnswer(poll_answer) => poll_answer
                .voter
                .chat()
                .map(|c| c.id)
                .or(poll_answer.voter.user().map(|u| ChatId::from(u.id)))
                .unwrap(),
            err => todo!("{err:#?}"),
        });
        UpdateWithSuppliedChatId(update, id)
    }
}

impl GetChatId for UpdateWithSuppliedChatId {
    fn chat_id(&self) -> Option<ChatId> {
        Some(self.1)
    }
}

pub async fn unhandled(update: Update) -> R {
    dbg!(update);
    Ok(())
}

pub fn is_command<'a>(cmd: &'static str) -> Handler<'a, R, DpHandlerDescription> {
    dptree::filter(move |m: Message| {
        m.text()
            .is_some_and(|t| &t.trim()[..1] == "/" && &t.trim()[1..] == cmd)
    })
}

pub fn if_is_command<'a, FnArgs, F: Injectable<R, FnArgs> + Send + Sync + 'a>(
    cmd: &'static str,
    handler: F,
) -> Handler<'a, R, DpHandlerDescription> {
    is_command(cmd).endpoint(handler)
}
