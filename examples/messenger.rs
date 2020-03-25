use rand::{distributions::Alphanumeric, prelude::*};
use std::sync::Arc;
use tbot::{
    connectors::Connector,
    contexts::fields,
    prelude::*,
    state::Chats,
    types::{chat, parameters::Text},
    Bot,
};
use tokio::sync::RwLock;

#[derive(Default)]
struct State(RwLock<Chats<String>>);

impl State {
    async fn participants(&self, room: &str) -> Vec<chat::Id> {
        self.0
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

    async fn join<C: Connector>(
        &self,
        bot: &Bot<C>,
        participant: chat::Id,
        room: String,
    ) {
        self.notify(
            bot,
            &room,
            Text::markdown_v2("_A participant has joined the room\\._"),
        )
        .await;

        let previous_room =
            self.0.write().await.insert_by_id(participant, room);

        if let Some(room) = previous_room {
            self.notify(
                bot,
                &room,
                Text::markdown_v2("_A participant has left the room\\._"),
            )
            .await;
        }
    }

    async fn notify<C: Connector>(
        &self,
        bot: &Bot<C>,
        room: &str,
        message: Text<'_>,
    ) {
        let participants = self.participants(room).await;

        for id in participants {
            let call_result = bot.send_message(id, message).call().await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    }
}

async fn broadcast<Ctx, Con>(context: Arc<Ctx>, state: Arc<State>)
where
    Ctx: fields::Text<Con>,
    Con: Connector,
{
    let chats = state.0.read().await;
    let room = chats.get(&*context);
    let sender_id = context.chat().id;

    if let Some(room) = room {
        let recipients = state.participants(room).await;

        for id in recipients {
            if id == sender_id {
                continue;
            }
            let call_result = context
                .bot()
                .send_message(id, &context.text().value)
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
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
                .send_message(Text::markdown_v2(
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

    bot.command("leave", |context, state| async move {
        let room = state.0.write().await.remove(&*context);

        if let Some(room) = room {
            state
                .notify(
                    &context.bot,
                    &room,
                    Text::markdown_v2("_A participant has left the room\\._"),
                )
                .await;
        }

        let call_result = context
            .send_message(Text::markdown_v2("_You have left the room\\._"))
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
                .collect::<String>();

            let message = format!(
                "_You have created a new room\\. Share this link so others \
                 can join your room:_ t\\.me/{}?start\\={}",
                username.replace("_", "\\_"),
                room
            );

            state.join(&context.bot, context.chat.id, room).await;

            let call_result = context
                .send_message(Text::markdown_v2(&message))
                .call()
                .await;

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
