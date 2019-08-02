use tbot::{
    prelude::*,
    types::keyboard::{
        self,
        inline::{Button, ButtonKind, Markup},
    },
};

const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

fn main() {
    let keyboard: Markup = vec![
        vec![
            Button::new("Cool!", ButtonKind::callback_data("cool")).into(),
            Button::new("Amazing!", ButtonKind::callback_data("amazing"))
                .into(),
        ]
        .into(),
        vec![Button::new(
            "I wanna get started with it!",
            ButtonKind::url(TUTORIAL),
        )
        .into()]
        .into(),
    ]
    .into();
    let keyboard: keyboard::Any = keyboard.into();
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
