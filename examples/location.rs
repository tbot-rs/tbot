use std::time::Duration;
use tbot::prelude::*;
use tokio::timer::delay_for;

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

#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.command("location", |context| {
        async move {
            let mut places = PLACES.iter().cycle();

            let location = context
                .send_location(*places.next().unwrap())
                .live_period(UPDATE_PERIOD)
                .call()
                .await
                .unwrap();

            for &place in places {
                delay_for(Duration::from_secs(INTERVAL)).await;

                let result = context
                    .edit_message_location(location.id, place)
                    .call()
                    .await;

                if result.is_err() {
                    break;
                }
            }
        }
    });

    bot.polling().start().await.unwrap();
}
