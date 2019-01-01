use tbot::{
    prelude::*,
    types::{Animation, Photo},
    Bot,
};

const CHAT: i64 = 0;

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let photo = bot
        .send_photo(CHAT, Photo::file(include_bytes!("./assets/photo.jpg")))
        .into_future()
        .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

    let animation = bot
        .send_animation(
            CHAT,
            Animation::file(include_bytes!("./assets/gif.mp4")),
        )
        .into_future()
        .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

    tbot::run(photo.join(animation));
}
