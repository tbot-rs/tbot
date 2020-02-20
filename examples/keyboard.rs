use tbot::{
    prelude::*,
    types::keyboard::inline::{Button, ButtonKind},
    Bot,
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
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("keyboard", |context| async move {
        let call_result = context
            .send_message("This is a keyboard done with tbot!")
            .reply_markup(KEYBOARD)
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.data_callback(|context| async move {
        let message = match context.data.as_str() {
            "cool" => "You're cool too!",
            "amazing" => "Thanks, I'm trying!",
            _ => "Are you trying to hack me?",
        };

        let call_result = context.notify(message).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
