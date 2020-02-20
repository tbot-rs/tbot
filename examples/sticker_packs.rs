use tbot::{types::input_file::PngSticker, Bot};

const USER: i64 = 0;
// Must end with `_by_<bot_username>`
const NAME: &str = "tbot";
const TITLE: &str = "tbot";
const STICKERS: [(&[u8], &str); 2] = [
    (include_bytes!("./assets/stickers/1.png"), "⌨️"),
    (include_bytes!("./assets/stickers/2.png"), "🐱"),
];

#[tokio::main]
async fn main() -> Result<(), tbot::errors::MethodCall> {
    let bot = Bot::from_env("BOT_TOKEN");

    let user_id = USER.into();
    let mut stickers = STICKERS.iter();
    let &(bytes, emoji) = stickers.next().unwrap();

    let sticker = PngSticker::bytes(bytes);
    bot.create_new_sticker_set(user_id, NAME, TITLE, sticker, emoji)
        .call()
        .await?;

    for &(bytes, emoji) in stickers {
        let sticker = PngSticker::bytes(bytes);
        bot.add_sticker_to_set(user_id, NAME, sticker, emoji)
            .call()
            .await?;
    }

    println!(
        "Go check out this amazing sticker pack: https://t.me/addstickers/{}",
        NAME,
    );

    Ok(())
}
