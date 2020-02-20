use std::{collections::HashMap, sync::Arc};
use tbot::{errors, prelude::*, Bot};
use tokio::sync::Mutex;

const GAME: &str = "";
const GOOD_PHRASE: &str = "tbot good";
const BAD_PHRASE: &str = "tbot bad";
const GOOD_MULTIPLIER: i32 = 10;
const BAD_MULTIPLIER: i32 = 100;

const SCORE_NOT_MODIFIED: &str = "Bad Request: BOT_SCORE_NOT_MODIFIED";

#[tokio::main]
async fn main() {
    let chats = Arc::new(Mutex::new(HashMap::new()));
    let game_chats_ref = Arc::clone(&chats);

    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("game", move |context| {
        let chats = Arc::clone(&game_chats_ref);
        async move {
            let call_result = context.send_game(GAME).call().await;
            let message = match call_result {
                Ok(message) => message,
                Err(err) => {
                    dbg!(err);
                    return;
                }
            };

            chats.lock().await.insert(message.chat.id, message.id);
        }
    });

    bot.text(move |context| {
        let chats = Arc::clone(&chats);
        async move {
            let message_id = {
                let chats = chats.lock().await;

                match chats.get(&context.chat.id) {
                    Some(id) => *id,
                    None => return,
                }
            };

            let user = match &context.from {
                Some(user) => user,
                None => return,
            };

            let text = context.text.value.to_lowercase();
            let good_score = text.matches(GOOD_PHRASE).count() as i32;
            let bad_score = text.matches(BAD_PHRASE).count() as i32;
            let score =
                GOOD_MULTIPLIER * good_score - BAD_MULTIPLIER * bad_score;
            let score = score.max(1) as u32;

            let call_result = context
                .set_message_game_score(message_id, user.id, score)
                .force(true)
                .call()
                .await;

            match call_result {
                Ok(_) => (),
                Err(errors::MethodCall::RequestError {
                    ref description,
                    ..
                }) if description == SCORE_NOT_MODIFIED => (),
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    });

    bot.polling().start().await.unwrap();
}
