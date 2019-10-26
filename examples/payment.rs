use tbot::{
    prelude::*,
    types::{
        parameters::{Flexibility::Flexible, Photo, Text},
        shipping, LabeledPrice,
    },
};

const PAYLOAD: &str = "crab";
const START_PARAMETER: &str = "crab";
const TITLE: &str = "A crab";
const PHOTO: Photo =
    Photo::new("https://www.rustacean.net/assets/rustacean-flat-happy.png");
const DESCRIPTION: &str = "Have you ever come across a heisenbug in your \
program? No more! Our crab will take all bugs out of your program for you.";
const CURRENCY: &str = "USD";
const PRICE: &[LabeledPrice] = &[LabeledPrice::new(TITLE, 1_00)];
const DELIVERY: &[shipping::Option] = &[shipping::Option::new(
    "crab",
    "Delivery Crab",
    &[LabeledPrice::new("At your home", 1_00)],
)];
const SUCCESS: &str = "Thanks! Your crab is already on its way.";

#[tbot::main]
async fn main() {
    // I don't want everyone to set up another environment variable
    // so they don't see constant errors from RLS or `cargo test`.
    let provider_token: &str = option_env!("PROVIDER_TOKEN").unwrap();
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.start(move |context| {
        let start_message =
            format!("Send `/start {}` to get started", START_PARAMETER);
        async move {
            if context.text.value == START_PARAMETER {
                context
                    .bot
                    .send_invoice(
                        context.chat.id,
                        TITLE,
                        DESCRIPTION,
                        PAYLOAD,
                        provider_token,
                        START_PARAMETER,
                        CURRENCY,
                        PRICE,
                    )
                    .photo(PHOTO)
                    .flexibility(Flexible)
                    .call()
                    .await
                    .unwrap();
            } else {
                let text = Text::markdown(&start_message);
                context.send_message(text).call().await.unwrap();
            };
        }
    });

    bot.shipping(|context| {
        async move {
            context.ok(DELIVERY).call().await.unwrap();
        }
    });

    bot.pre_checkout(|context| {
        async move {
            context.ok().call().await.unwrap();
        }
    });

    bot.payment(|context| {
        async move {
            context.send_message(SUCCESS).call().await.unwrap();
        }
    });

    bot.polling().start().await.unwrap();
}
