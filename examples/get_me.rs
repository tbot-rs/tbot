use tbot::{prelude::*, Bot};

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let get_me = bot
        .get_me()
        .into_future()
        .map_err(|error| {
            dbg!(error);
        })
        .map(|me| {
            dbg!(me);
        });

    tbot::run(get_me);
}
