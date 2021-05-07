use rand::{distributions::Alphanumeric, prelude::*};
use std::sync::Arc;
use tbot::{
    contexts::fields,
    markup::{italic, markdown_v2},
    prelude::*,
    state::{
        messages::{MessageId, Messages},
        Chats,
    },
    types::{chat, parameters::Text},
    util::entities,
    Bot,
};
use tokio::sync::RwLock;

#[derive(Default)]
struct State {
    chats: RwLock<Chats<String>>,
    messages: RwLock<Messages<Vec<MessageId>>>,
    username: String,
}

impl State {
    fn new(username: String) -> Self {
        Self {
            chats: Default::default(),
            messages: Default::default(),
            username,
        }
    }

    async fn participants(&self, room: &str) -> Vec<chat::Id> {
        let chats = self.chats.read().await;
        let participants = chats.iter().filter_map(|(id, chat_room)| {
            if chat_room == room {
                Some(id)
            } else {
                None
            }
        });
        participants.collect()
    }

    async fn join(&self, bot: &Bot, participant: chat::Id, room: String) {
        self.notify(
            bot,
            &room,
            markdown_v2(italic("A participant has joined the room.")),
        )
        .await;

        let previous_room =
            self.chats.write().await.insert_by_id(participant, room);

        if let Some(room) = previous_room {
            self.notify(
                bot,
                &room,
                markdown_v2(italic("A participant has left the room.")),
            )
            .await;
        }
    }

    async fn notify(&self, bot: &Bot, room: &str, message: impl Into<Text>) {
        let participants = self.participants(room).await;
        let message = message.into();

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
        let text = markdown_v2(entities(context.text()));

        for id in recipients {
            let call_result =
                context.bot().send_message(id, text.clone()).call().await;

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
        let text = markdown_v2(entities(context.text()));

        for MessageId {
            chat_id,
            message_id,
        } in messages
        {
            let call_result = context
                .bot()
                .edit_message_text(*chat_id, *message_id, text.clone())
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

    let mut bot = bot.stateful_event_loop(State::new(username));

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
                .send_message(markdown_v2(italic("You have joined the room.")))
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
                 — /send — send messages (you can omit the command if you \
                   don't need to send a command in the beginning of your \
                   message);\n\
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
                    markdown_v2(italic("A participant has left the room.")),
                )
                .await;
        }

        let call_result = context
            .send_message(markdown_v2(italic("You have left the room.")))
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("create_room", |context, state| async move {
        let room = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        let message = markdown_v2((
            italic(
                "You have created a new room. Share this link so others can\
                 join your room:",
            ),
            format!(" t.me/{}?start={}", state.username, room),
        ));

        state.join(&context.bot, context.chat.id, room).await;

        let call_result = context.send_message(message).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
