use tbot::{
    prelude::*,
    types::{InlineButton, InlineButtonType, InlineKeyboard},
    Bot,
};

const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");

    let request = bot
        // put your ID here
        .send_message(0, "This is a keyboard done with tbot!")
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
        .map_err(|error| println!("Whops, an error happened: {:#?}", error));

    tbot::run(request);
}
