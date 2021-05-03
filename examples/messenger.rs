use rand::{distributions::Alphanumeric, prelude::*};
use std::sync::Arc;
use tbot::{
    contexts::fields,
    prelude::*,
    state::{
        messages::{MessageId, Messages},
        Chats,
    },
    types::{chat, parameters::Text},
    Bot,
};
use tokio::sync::RwLock;

#[derive(Default)]
struct State {
    chats: RwLock<Chats<String>>,
    messages: RwLock<Messages<Vec<MessageId>>>,
}

impl State {
    async fn participants(&self, room: &str) -> Vec<chat::Id> {
        self.chats
            .read()
            .await
            .iter()
            .filter_map(
                |(id, chat_room)| {
                    if chat_room == room {
                        Some(id)
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>()
    }

    async fn join(&self, bot: &Bot, participant: chat::Id, room: String) {
        self.notify(
            bot,
            &room,
            Text::with_markdown_v2("_A participant has joined the room\\._"),
        )
        .await;

        let previous_room =
            self.chats.write().await.insert_by_id(participant, room);

        if let Some(room) = previous_room {
            self.notify(
                bot,
                &room,
                Text::with_markdown_v2("_A participant has left the room\\._"),
            )
            .await;
        }
    }

    async fn notify(&self, bot: &Bot, room: &str, message: Text) {
        let participants = self.participants(room).await;

        for id in participants {
            let call_result =
                bot.send_message(id, message.clone()).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    }
}

async fn broadcast<C>(context: Arc<C>, state: Arc<State>)
where
    C: fields::Text,
{
    let chats = state.chats.read().await;
    let room = chats.get(&*context);
    let sender_id = context.chat().id;

    if let Some(room) = room {
        let mut recipients = state.participants(room).await;
        recipients.retain(|&id| id != sender_id);
        let mut sent_messages = Vec::with_capacity(recipients.len());

        for id in recipients {
            let call_result = context
                .bot()
                .send_message(id, &context.text().value)
                .call()
                .await;

            match call_result {
                Ok(message) => {
                    sent_messages.push(MessageId::from_message(&message));
                }
                Err(err) => {
                    dbg!(err);
                }
            }
        }

        state
            .messages
            .write()
            .await
            .insert(&*context, sent_messages);
    } else {
        let call_result = context
            .send_message(
                "You have not joined a room to send messages. \
                 Join one or create a room with /create_room.",
            )
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    }
}

async fn broadcast_edit<C>(context: Arc<C>, state: Arc<State>)
where
    C: fields::Text,
{
    if let Some(messages) = state.messages.read().await.get(&*context) {
        for MessageId {
            chat_id,
            message_id,
        } in messages
        {
            let call_result = context
                .bot()
                .edit_message_text(*chat_id, *message_id, &context.text().value)
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env("BOT_TOKEN");
    let me = bot.get_me().call().await.ok();
    let username = me
        .and_then(|me| me.user.username)
        .expect("Could not get username");

    let mut bot = bot.stateful_event_loop(State::default());

    bot.start(|context, state| async move {
        if context.text.value.is_empty() {
            let call_result = context
                .send_message(
                    "Hello! I'm a bot for anonymous messaging in rooms. \
                     Use the /create_room command to create a new room.",
                )
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        } else {
            state
                .join(
                    &context.bot,
                    context.chat.id,
                    context.text.value.to_owned(),
                )
                .await;

            let call_result = context
                .send_message(Text::with_markdown_v2(
                    "_You have joined the room\\._",
                ))
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.help(|context, _| async move {
        let call_result = context
            .send_message(
                "Here are the commands I know:\n\n\

                — /create_room — create rooms;\n\
                — /send — send messages (you can omit the command if you don't \
                need to send a command in the beginning of your message);\n\
                — /leave — leave the current room;\n\
                — /help — send this message.",
            )
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("send", broadcast);
    bot.text(broadcast);

    bot.edited_command("send", broadcast_edit);
    bot.edited_text(broadcast_edit);

    bot.command("leave", |context, state| async move {
        let room = state.chats.write().await.remove(&*context);

        if let Some(room) = room {
            state
                .notify(
                    &context.bot,
                    &room,
                    Text::with_markdown_v2(
                        "_A participant has left the room\\._",
                    ),
                )
                .await;
        }

        let call_result = context
            .send_message(Text::with_markdown_v2("_You have left the room\\._"))
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("create_room", move |context, state| {
        let username = username.clone();
        async move {
            let room = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>();

            let message = format!(
                "_You have created a new room\\. Share this link so others \
                 can join your room:_ t\\.me/{}?start\\={}",
                username.replace("_", "\\_"),
                room
            );

            state.join(&context.bot, context.chat.id, room).await;

            let call_result = context
                .send_message(Text::with_markdown_v2(&message))
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
