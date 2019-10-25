use tbot::{
    prelude::*,
    types::input_file::{Animation, Document, Photo, Video},
};

const PHOTO: &[u8] = include_bytes!("./assets/photo.jpg");
const GIF: &[u8] = include_bytes!("./assets/gif.mp4");
const TUTORIAL: &[u8] = include_bytes!("./tutorial.rs");

#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("photo", |context| {
        tokio::spawn(async move {
            let photo = Photo::bytes(PHOTO);
            context.send_photo(photo).call().await.unwrap();
        });
    });

    bot.command("animation", |context| {
        tokio::spawn(async move {
            let animation = Animation::bytes(GIF);
            context.send_animation(animation).call().await.unwrap();
        });
    });

    bot.command("document", |context| {
        tokio::spawn(async move {
            let document = Document::bytes("tutorial.rs", TUTORIAL);
            context.send_document(document).call().await.unwrap();
        });
    });

    // Because the video for this example is silent, Telegram will send it as a
    // gif. You can try sending a video with sound and it will be sent as a
    // video. Also, Telegram seems not to create the thumb and figure out the
    // duration on its own, so the video might look somewhat corrupted at first.
    bot.command("video", |context| {
        tokio::spawn(async move {
            let video = Video::bytes(GIF);
            context.send_video(video).call().await.unwrap();
        });
    });

    bot.command("album", |context| {
        tokio::spawn(async move {
            let album = &[Photo::bytes(PHOTO).into(), Video::bytes(GIF).into()];
            context.send_media_group(album).call().await.unwrap();
        });
    });

    bot.polling().start().await.unwrap();
}
