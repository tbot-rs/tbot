use tbot::{
    prelude::*,
    types::keyboard::inline::{Button, ButtonKind},
};

const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";
const KEYBOARD: &[&[Button]] = &[
    &[
        Button::new("Cool!", ButtonKind::CallbackData("cool")),
        Button::new("Amazing!", ButtonKind::CallbackData("amazing")),
    ],
    &[Button::new(
        "I wanna get started with it!",
        ButtonKind::Url(TUTORIAL),
    )],
];

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.command("keyboard", |context| {
        let message = context
            .send_message("This is a keyboard done with tbot!")
            .reply_markup(KEYBOARD)
            .into_future()
            .map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(message);
    });

    bot.data_callback(|context| {
        let message = match context.data.as_str() {
            "cool" => "You're cool too!",
            "amazing" => "Thanks, I'm trying!",
            _ => "Are you trying to hack me?",
        };

        let answer = context.notify(message).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(answer);
    });

    bot.polling().start();
}
