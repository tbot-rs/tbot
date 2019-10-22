use tbot::types::input_file::PngSticker;

const USER: i64 = 0;
// Must end with `_by_<bot_username>`
const NAME: &str = "tbot";
const TITLE: &str = "tbot";
const STICKERS: [(&[u8], &str); 2] = [
    (include_bytes!("./assets/stickers/1.png"), "⌨️"),
    (include_bytes!("./assets/stickers/2.png"), "🐱"),
];

#[tbot::main]
async fn main() {
    let bot = tbot::from_env!("BOT_TOKEN");

    let mut stickers = STICKERS.iter();
    let first_sticker = stickers.next().unwrap();

    bot.create_new_sticker_set(
        USER.into(),
        NAME,
        TITLE,
        PngSticker::bytes(first_sticker.0),
        first_sticker.1,
    )
    .call()
    .await
    .unwrap();

    for sticker in stickers {
        bot.add_sticker_to_set(
            USER.into(),
            NAME,
            PngSticker::bytes(sticker.0),
            sticker.1,
        )
        .call()
        .await
        .unwrap();
    }

    println!(
        "Go check out this amazing sticker pack: https://t.me/addstickers/{}",
        NAME,
    );
}
