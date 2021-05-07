use tbot::{
    markup::{inline_code, markdown_v2},
    prelude::*,
    types::{parameters::Photo, shipping, LabeledPrice},
    Bot,
};

const PAYLOAD: &str = "crab";
const START_PARAMETER: &str = "crab";
const TITLE: &str = "A crab";
const DESCRIPTION: &str = "Have you ever come across a heisenbug in your \
program? No more! Our crab will take all bugs out of your program for you.";
const CURRENCY: &str = "USD";
const SUCCESS: &str = "Thanks! Your crab is already on its way.";

#[tokio::main]
async fn main() {
    // The one that you'd get from BotFather after connecting a payment provider
    // to your bot.
    let provider_token = std::env::var("PROVIDER_TOKEN")
        .expect("the PROVIDER_TOKEN env variable was not specified");
    let mut bot =
        Bot::from_env("BOT_TOKEN").stateful_event_loop(provider_token);

    bot.start(|context, provider_token| async move {
        let call_result = if context.text.value == START_PARAMETER {
            let price = [LabeledPrice::new(TITLE, 1_00)];
            let mut invoice = context.send_invoice(
                TITLE,
                DESCRIPTION,
                PAYLOAD,
                &*provider_token,
                START_PARAMETER,
                CURRENCY,
                price,
            );
            let photo = Photo::new(
                "https://www.rustacean.net/assets/rustacean-flat-happy.png",
            );
            invoice = invoice.photo(photo).is_flexible(true);

            invoice.call().await
        } else {
            let error_message = markdown_v2((
                "Send ",
                inline_code(format!("/start {}", START_PARAMETER)),
                " to get started",
            ));
            context.send_message(error_message).call().await
        };

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.shipping(|context, _| async move {
        let price = [LabeledPrice::new("At your home", 1_00)];
        let delivery = [shipping::Option::new("crab", "Delivery Crab", price)];

        let call_result = context.ok(delivery).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.pre_checkout(|context, _| async move {
        let call_result = context.ok().call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.payment(|context, _| async move {
        let call_result = context.send_message(SUCCESS).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
