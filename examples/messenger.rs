use std::collections::HashMap;
use tbot::{
    prelude::*,
    types::{
        chat,
        message::text::{Entity, EntityKind},
        parameters::{ChatId, Text},
    },
    Bot,
};
use tokio::sync::RwLock;

#[derive(PartialEq, Eq, Hash)]
enum Recipient {
    Id(chat::Id),
    Username(String),
}

impl<'a> From<ChatId<'a>> for Recipient {
    fn from(chat_id: ChatId<'a>) -> Self {
        match chat_id {
            ChatId::Id(id) => Self::Id(id),
            ChatId::Username(username) => Self::Username(username.to_owned()),
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct State {
    messages: RwLock<HashMap<Recipient, Vec<String>>>,
}

#[tokio::main]
async fn main() {
    let mut bot =
        Bot::from_env("BOT_TOKEN").stateful_event_loop(State::default());

    bot.start(|context, _| async move {
        context
            .send_message_in_reply(
                "Hello! I'm a bot that can transfer messages between people. \
                Use /send_message to send messages and /last_message to see \
                the last message sent to you.",
            )
            .call()
            .await
            .unwrap();
    });

    bot.command("send_message", |context, state| async move {
        let recipient;
        let message;

        match context.text.entities.get(0) {
            Some(Entity {
                kind: EntityKind::Mention,
                offset: 0,
                length,
                ..
            }) => {
                recipient = ChatId::Username(&context.text.value[1..*length]);
                message = context.text.value[*length..].trim();
            }
            Some(Entity {
                kind: EntityKind::TextMention(user),
                offset: 0,
                length,
                ..
            }) => {
                recipient = ChatId::Id(chat::Id(user.id.0));
                message = context.text.value[*length..].trim();
            }
            _ => {
                let message = Text::markdown_v2(
                    "Please specify the recipient like this: \
                    `/send_message @username <text>`",
                );
                context.send_message_in_reply(message).call().await.unwrap();
                return;
            }
        };

        state
            .messages
            .write()
            .await
            .entry(recipient.into())
            .or_insert_with(Vec::new)
            .push(message.to_owned());

        context
            .send_message_in_reply(
                "Good! Now tell the recipient about me, and they'll see \
                 your message using the /last_message command.",
            )
            .call()
            .await
            .unwrap();
    });

    bot.command("last_message", |context, state| async move {
        let from = match &context.from {
            Some(from) => from,
            None => return,
        };
        let mut state = state.messages.write().await;
        let mut messages = match &from.username {
            None => {
                let id = chat::Id(from.id.0);
                state.get_mut(&Recipient::Id(id))
            }
            Some(username) => {
                state.get_mut(&Recipient::Username(username.clone()))
            }
        };

        let last_message =
            messages.as_mut().and_then(|messages| messages.pop());

        if let Some(message) = last_message {
            let message = format!("The last message to you:\n\n{}", message);
            context
                .send_message_in_reply(&message)
                .call()
                .await
                .unwrap();
        } else {
            context
                .send_message_in_reply(
                    "You have not received any messages yet.",
                )
                .call()
                .await
                .unwrap();
        }
    });

    bot.polling().start().await.unwrap();
}
