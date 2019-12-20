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

#[tokio::main]
async fn main() {
    let provider_token = option_env!("PROVIDER_TOKEN").unwrap();
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.start(move |context| {
        async move {
            let call_result = if context.text.value == START_PARAMETER {
                let mut invoice = context.send_invoice(
                    TITLE,
                    DESCRIPTION,
                    PAYLOAD,
                    provider_token,
                    START_PARAMETER,
                    CURRENCY,
                    PRICE,
                );
                invoice = invoice.photo(PHOTO).flexibility(Flexible);

                invoice.call().await
            } else {
                let error_message =
                    format!("Send `/start {}` to get started", START_PARAMETER);
                let text = Text::markdown(&error_message);
                context.send_message(text).call().await
            };

            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.shipping(|context| {
        async move {
            let call_result = context.ok(DELIVERY).call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.pre_checkout(|context| {
        async move {
            let call_result = context.ok().call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.payment(|context| {
        async move {
            let call_result = context.send_message(SUCCESS).call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
