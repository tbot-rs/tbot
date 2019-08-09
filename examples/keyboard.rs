use tbot::{
    prelude::*,
    types::keyboard::{
        self,
        inline::{Button, ButtonKind},
    },
};

const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

fn main() {
    // If we created a keyboard of the actual type that `tbot` needs, it would
    // be less readable. Though `tbot` can convert this vector on its own
    // (i.e. if you declared it without the final `into`), it would be
    // ineffecient as it would have to convert it each time the keyboard
    // is requested. Instead, we convert the keyboard into `keyboard::Any`
    // once, so when the keyboard is requested, it's already in the appropriate
    // type.
    let keyboard: keyboard::Any = vec![
        vec![
            Button::new("Cool!", ButtonKind::callback_data("cool")),
            Button::new("Amazing!", ButtonKind::callback_data("amazing")),
        ],
        vec![Button::new(
            "I wanna get started with it!",
            ButtonKind::url(TUTORIAL),
        )],
    ]
    .into();

    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.command("keyboard", move |context| {
        let message = context
            .send_message("This is a keyboard done with tbot!")
            .reply_markup(&keyboard)
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
