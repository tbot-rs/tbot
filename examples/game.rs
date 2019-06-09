//! Simple but stupid game. Note that it's implemented

use std::sync::{Arc, Mutex};
use tbot::prelude::*;

const CHAT: i64 = 0;
const GAME: &str = "";
const GOOD_PHRASE: &str = "tbot good";
const BAD_PHRASE: &str = "tbot bad";
const GOOD_MULTIPLIER: i32 = 10;
const BAD_MULTIPLIER: i32 = 100;

const SCORE_NOT_MODIFIED: &str = "Bad Request: BOT_SCORE_NOT_MODIFIED";

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN");

    let message_id = Arc::new(Mutex::new(None));
    let on_ok = Arc::clone(&message_id);

    let game = bot
        .send_game(CHAT, GAME)
        .into_future()
        .map(move |message| {
            *on_ok.lock().unwrap() = Some(message.id);
        })
        .map_err(|error| {
            dbg!(error);
        });

    tbot::run(game);

    let message_id =
        Arc::try_unwrap(message_id).unwrap().into_inner().unwrap().unwrap();

    bot.text(move |context| {
        if let Some(user) = &context.from {
            let text = context.text.value.to_lowercase();
            let good_score = text.matches(GOOD_PHRASE).count() as i32;
            let bad_score = text.matches(BAD_PHRASE).count() as i32;
            let score =
                GOOD_MULTIPLIER * good_score - BAD_MULTIPLIER * bad_score;

            let update = context
                .set_message_game_score(
                    message_id,
                    user.id,
                    score.max(1) as u32,
                )
                .force(true)
                .into_future()
                .map_err(|error| {
                    if let tbot::methods::DeliveryError::RequestError {
                        description,
                        ..
                    } = &error
                    {
                        if description != SCORE_NOT_MODIFIED {
                            dbg!(error);
                        }
                    } else {
                        dbg!(error);
                    }
                });

            tbot::spawn(update);
        }
    });

    bot.polling().start();
}
