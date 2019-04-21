use tbot::{
    prelude::*,
    types::input_file::{Animation, Document, Photo, Video},
};

const CHAT: i64 = 0;
const PHOTO: &[u8] = include_bytes!("./assets/photo.jpg");
const GIF: &[u8] = include_bytes!("./assets/gif.mp4");
const TUTORIAL: &[u8] = include_bytes!("./tutorial.rs");

fn main() {
    let bot = tbot::bot!("BOT_TOKEN");

    let photo = bot.send_photo(CHAT, &Photo::bytes(PHOTO)).into_future();
    let animation =
        bot.send_animation(CHAT, &Animation::bytes(GIF)).into_future();
    let document = bot
        .send_document(CHAT, &Document::bytes("tutorial.rs", TUTORIAL))
        .into_future();

    // Because the video for this example is silent, Telegram will send it as a
    // gif. You can try sending a video with sound and it will be sent as a
    // video. Also, Telegram seems not to create the thumb and figure out the
    // duration on its own, so the video might look somewhat corrupted at first.
    let video = bot.send_video(CHAT, &Video::bytes(GIF)).into_future();
    let album = bot
        .send_media_group(
            CHAT,
            vec![Photo::bytes(PHOTO).into(), Video::bytes(GIF).into()],
        )
        .into_future();

    let joined =
        photo.join5(animation, document, video, album).map_err(|error| {
            dbg!(error);
        });

    tbot::run(joined);
}
