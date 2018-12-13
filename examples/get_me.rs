use tbot::{prelude::*, Bot};

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let request = bot
        .get_me()
        .into_future()
        .map_err(|error| println!("Oops, an error happened: {:#?}", error))
        .map(|me| println!("Here I am: {:#?}", me));

    tbot::run(request);
}
