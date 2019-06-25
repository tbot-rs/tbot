use futures::future::Either;
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

fn main() {
    // I don't want everyone to set up another environment variable
    // so they don't see constant errors from RLS or `cargo test`.
    let provider_token: &str = option_env!("PROVIDER_TOKEN").unwrap();
    let start_message =
        format!("Send `/start {}` to get started", START_PARAMETER);
    let mut bot = tbot::bot!("BOT_TOKEN").event_loop();

    bot.start(move |context| {
        let reply = if context.text.value == START_PARAMETER {
            let invoice = context
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
                .flexibility(Flexible);

            Either::A(invoice.into_future())
        } else {
            let reply = context.send_message(Text::markdown(&start_message));
            Either::B(reply.into_future())
        };

        tbot::spawn(reply.map_err(|err| {
            dbg!(err);
        }));
    });

    bot.shipping(|context| {
        let report = context.ok(DELIVERY).into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(report);
    });

    bot.pre_checkout(|context| {
        let report = context.ok().into_future().map_err(|err| {
            dbg!(err);
        });

        tbot::spawn(report);
    });

    bot.payment(|context| {
        let message =
            context.send_message(SUCCESS).into_future().map_err(|err| {
                dbg!(err);
            });

        tbot::spawn(message);
    });

    bot.polling().start();
}
