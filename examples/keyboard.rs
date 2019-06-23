use tbot::{
    prelude::*,
    types::{
        chat,
        keyboard::inline::{Button, ButtonKind},
    },
};

const CHAT: chat::Id = chat::Id(0);
const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

fn main() {
    let bot = tbot::bot!("BOT_TOKEN");

    let message = bot
        .send_message(CHAT, "This is a keyboard done with tbot!")
        .reply_markup(
            &[
                &[
                    Button::new("Cool!", ButtonKind::CallbackData("cool")),
                    Button::new(
                        "Amazing!",
                        ButtonKind::CallbackData("amazing"),
                    ),
                ][..],
                &[Button::new(
                    "I wanna get started with it!",
                    ButtonKind::Url(TUTORIAL),
                )],
            ][..],
        )
        .into_future()
        .map_err(|err| {
            dbg!(err);
        });

    tbot::run(message);
}
