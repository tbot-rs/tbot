use std::{sync::Arc, time::Duration};
use tbot::{connectors::Connector, contexts::Text, prelude::*};
use tokio::time::delay_for;

const INTERVAL: u64 = 15;
const PLACES: [(f64, f64); 6] = [
    (38.904722, -77.016389), // Washington
    (51.507222, -0.1275),    // London
    (41.9, 12.5),            // Rome
    (59.329444, 18.068611),  // Stockholm
    (55.796389, 49.108889),  // Kazan
    (56.5, 84.966667),       // Tomsk
];
const UPDATE_PERIOD: u32 = 3600 * 24;

#[tokio::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("location", handle_location);

    bot.polling().start().await.unwrap();
}

async fn handle_location<C: Connector>(context: Arc<Text<C>>) {
    let mut places = PLACES.iter().cycle();

    let first_place = *places.next().unwrap();
    let call_result = context
        .send_location(first_place)
        .live_period(UPDATE_PERIOD)
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
        delay_for(Duration::from_secs(INTERVAL)).await;

        let call_result = context
            .edit_message_location(location.id, place)
            .call()
            .await;

        if call_result.is_err() {
            break;
        }
    }
}
