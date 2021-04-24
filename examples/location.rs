use std::{sync::Arc, time::Duration};
use tbot::{
    contexts::Command, prelude::*, types::parameters::LiveLocation, Bot,
};
use tokio::time::sleep;

const INTERVAL: u64 = 15;
const PLACES: [(f64, f64); 7] = [
    (38.904_722, -77.016_389),    // Washington
    (51.507_222, -0.1275),        // London
    (41.9, 12.5),                 // Rome
    (59.329_444, 18.068_611),     // Stockholm
    (41.709_539_2, 44.802_765_4), // Tbilisi
    (55.796_389, 49.108_889),     // Kazan
    (56.5, 84.966_667),           // Tomsk
];
const UPDATE_PERIOD: u32 = 3600 * 24;

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.command("location", handle_location);

    bot.polling().start().await.unwrap();
}

async fn handle_location(context: Arc<Command>) {
    let mut places = PLACES.iter().cycle();

    let first_place = *places.next().unwrap();
    let call_result = context
        .send_location(first_place)
        .live_location(LiveLocation::new(UPDATE_PERIOD))
        .call()
        .await;
    let location = match call_result {
        Ok(location) => location,
        Err(err) => {
            dbg!(err);
            return;
        }
    };

    for &place in places {
        sleep(Duration::from_secs(INTERVAL)).await;

        let call_result = context
            .edit_message_location(location.id, place)
            .call()
            .await;

        if call_result.is_err() {
            break;
        }
    }
}
