extern crate tbot;

use tbot::{prelude::*, Bot};

fn main() {
    let token = std::env::var("BOT_TOKEN").unwrap();

    let bot = Bot::new(&token);

    let request = bot
        .get_me()
        .get_request()
        .map_err(|error| println!("Oops, an error happened: {:#?}", error))
        .map(|me| println!("Here I am: {:#?}", me));

    tbot::run(request);
}
