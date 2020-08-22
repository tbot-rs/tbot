use tbot::{
    prelude::*,
    types::keyboard::inline::{Button, ButtonKind},
    Bot,
};

const TUTORIAL: &str = "https://gitlab.com/SnejUgal/tbot/wikis/Tutorial";

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("keyboard", |context| async move {
        let keyboard: &[&[_]] = &[
            &[
                Button::new("Cool!", ButtonKind::with_callback_data("cool")),
                Button::new(
                    "Amazing!",
                    ButtonKind::with_callback_data("amazing"),
                ),
            ],
            &[Button::new(
                "I wanna get started with it!",
                ButtonKind::with_url(TUTORIAL),
            )],
        ];
        let call_result = context
            .send_message("This is a keyboard done with tbot!")
            .reply_markup(keyboard)
            .call()
            .await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.message_data_callback(|context| async move {
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
