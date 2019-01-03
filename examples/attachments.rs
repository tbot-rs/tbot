use tbot::{
    prelude::*,
    types::{Animation, Document, Photo, Video},
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

    let document = bot
        .send_document(
            CHAT,
            Document::file("tutorial.rs", include_bytes!("./tutorial.rs")),
        )
        .into_future()
        .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

    let video = bot
        .send_video(
            CHAT,
            // Note: because the video for this example is silent, Telegram
            // will send it as a gif. You can try sending a video wth sound and
            // it will be processed as a video.
            //
            // Also Telegram seems not to create the thumb and figure out the
            // duration on its own, so the video might look somewhat corrupted
            // at first.
            Video::file(include_bytes!("./assets/gif.mp4")),
        )
        .into_future()
        .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

    let album = bot
        .send_media_group(
            CHAT,
            vec![
                Photo::file(include_bytes!("./assets/photo.jpg")).into(),
                Video::file(include_bytes!("./assets/gif.mp4")).into(),
            ],
        )
        .into_future()
        .map_err(|error| eprintln!("Whops, got an error: {:#?}", error));

    tbot::run(photo.join5(animation, document, video, album));
}
