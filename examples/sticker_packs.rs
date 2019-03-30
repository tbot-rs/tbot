use futures::{stream::iter_ok, Stream};
use tbot::{prelude::*, types::input_file::PngSticker, Bot};

const USER: u64 = 0;
// Must end with `_by_<bot_username>`
const NAME: &str = "tbot";
const TITLE: &str = "tbot";
const STICKERS: [(&[u8], &str); 2] = [
    (include_bytes!("./assets/stickers/1.png"), "⌨️"),
    (include_bytes!("./assets/stickers/2.png"), "🐱"),
];

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let mut stickers = STICKERS.iter();
    let first_sticker = stickers.next().unwrap();

    let sticker_pack = bot
        .create_new_sticker_set(
            USER,
            NAME,
            TITLE,
            &PngSticker::bytes(first_sticker.0),
            first_sticker.1,
        )
        .into_future()
        .map_err(|error| {
            dbg!(error);
        })
        .and_then(|_| {
            iter_ok(stickers).for_each(move |sticker| {
                bot.add_sticker_to_set(
                    USER,
                    NAME,
                    &PngSticker::bytes(sticker.0),
                    sticker.1,
                )
                .into_future()
                .map_err(|error| {
                    dbg!(error);
                })
            })
        })
        .map(|_| {
            println!(
                "Go check out this amazing sticker pack: \
                 https://t.me/addstickers/{}",
                NAME,
            );
        });

    tbot::run(sticker_pack);
}
