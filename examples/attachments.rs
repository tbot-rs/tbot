use tbot::{
    prelude::*,
    types::input_file::{Animation, Document, GroupMedia, Photo, Video},
};

const PHOTO: &[u8] = include_bytes!("./assets/photo.jpg");
const GIF: &[u8] = include_bytes!("./assets/gif.mp4");
const TUTORIAL: &[u8] = include_bytes!("./tutorial.rs");

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.command("photo", |context| {
        let photo = Photo::bytes(PHOTO);
        let message = context.send_photo(photo).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(message);
    });

    bot.command("animation", |context| {
        let animation = Animation::bytes(GIF);
        let message =
            context.send_animation(animation).into_future().map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(message);
    });

    bot.command("document", |context| {
        let document = Document::bytes("tutorial.rs", TUTORIAL);
        let message =
            context.send_document(document).into_future().map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(message);
    });

    // Because the video for this example is silent, Telegram will send it as a
    // gif. You can try sending a video with sound and it will be sent as a
    // video. Also, Telegram seems not to create the thumb and figure out the
    // duration on its own, so the video might look somewhat corrupted at first.
    bot.command("video", |context| {
        let video = Video::bytes(GIF);
        let message = context.send_video(video).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(message);
    });

    bot.command("album", |context| {
        let photo: GroupMedia = Photo::bytes(PHOTO).into();
        let video: GroupMedia = Video::bytes(GIF).into();
        let album = [photo.into(), video.into()];
        let message =
            context.send_media_group(&album[..]).into_future().map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(message);
    });

    bot.polling().start();
}
