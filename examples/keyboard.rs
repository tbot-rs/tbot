use tbot::{
    prelude::*,
    types::{InlineButton, InlineButtonType, InlineKeyboard},
    Bot,
};

const CHAT: i64 = 0;
const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let message = bot
        .send_message(CHAT, "This is a keyboard done with tbot!")
        .reply_markup(InlineKeyboard::new(vec![
            vec![
                InlineButton::new(
                    "Cool!",
                    InlineButtonType::CallbackData("cool"),
                ),
                InlineButton::new(
                    "Amazing!",
                    InlineButtonType::CallbackData("amazing"),
                ),
            ],
            vec![InlineButton::new(
                "I wanna get started with it!",
                InlineButtonType::Url(TUTORIAL),
            )],
        ]))
        .into_future()
        .map_err(|error| {
            dbg!(error);
        });

    tbot::run(message);
}
