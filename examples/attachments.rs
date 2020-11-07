use tbot::{
    prelude::*,
    types::input_file::{Animation, Document, Photo, PhotoOrVideo, Video},
    Bot,
};

const PHOTO: &[u8] = include_bytes!("./assets/photo.jpg");
const GIF: &[u8] = include_bytes!("./assets/gif.mp4");
const TUTORIAL: &[u8] = include_bytes!("./tutorial.rs");

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("photo", |context| async move {
        let photo = Photo::with_bytes(PHOTO);
        let call_result = context.send_photo(photo).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("animation", |context| async move {
        let animation = Animation::with_bytes(GIF);
        let call_result = context.send_animation(animation).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("document", |context| async move {
        let document = Document::with_bytes("tutorial.rs", TUTORIAL);
        let call_result = context.send_document(document).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    // Because the video for this example is silent, Telegram will send it as a
    // gif. You can try sending a video with sound and it will be sent as a
    // video. Also, Telegram seems not to create the thumb and figure out the
    // duration on its own, so the video might look somewhat corrupted at first.
    bot.command("video", |context| async move {
        let video = Video::with_bytes(GIF);
        let call_result = context.send_video(video).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("album", |context| async move {
        let album: &[PhotoOrVideo] = &[
            Photo::with_bytes(PHOTO).into(),
            Video::with_bytes(GIF).into(),
        ];
        let call_result = context.send_media_group(album).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
