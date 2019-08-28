use tbot::prelude::*;

fn main() {
    let bot = tbot::from_env!("BOT_TOKEN");

    let get_me = bot
        .get_me()
        .into_future()
        .map_err(|err| {
            dbg!(err);
        })
        .map(|me| {
            dbg!(me);
        });

    tbot::run(get_me);
}
