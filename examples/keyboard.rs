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

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("keyboard", |context| {
        async move {
            context
                .send_message("This is a keyboard done with tbot!")
                .reply_markup(KEYBOARD)
                .call()
                .await
                .unwrap();
        }
    });

    bot.data_callback(|context| {
        async move {
            let message = match context.data.as_str() {
                "cool" => "You're cool too!",
                "amazing" => "Thanks, I'm trying!",
                _ => "Are you trying to hack me?",
            };

            context.notify(message).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();
}
