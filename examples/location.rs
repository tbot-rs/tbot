use std::time::{Duration, Instant};
use tbot::{prelude::*, Bot};
use tokio::{prelude::*, timer::Interval};

const INTERVAL: u16 = 15;
const PLACES: [(f64, f64); 6] = [
    (38.904722, -77.016389), // Washington
    (51.507222, -0.1275),    // London
    (41.9, 12.5),            // Rome
    (59.329444, 18.068611),  // Stockholm
    (55.796389, 49.108889),  // Kazan
    (56.5, 84.966667),       // Tomsk
];

fn main() {
    let bot = Bot::from_env("BOT_TOKEN");
    let mut places = PLACES.iter();

    let request = bot
        // Put your ID here
        .send_location(0, *places.next().unwrap())
        .live_period((PLACES.len() as u16 * INTERVAL).max(60))
        .into_future()
        .map_err(|error| eprintln!("Whops, an error happened: {:#?}", error))
        .and_then(|message| {
            Interval::new(Instant::now(), Duration::from_secs(INTERVAL as u64))
                .take(PLACES.len() as u64)
                .skip(1)
                .map_err(|error| {
                    eprintln!("Whops, an error happened: {:#?}", error);
                })
                .for_each(move |_| {
                    bot.edit_message_location(
                        message.chat.id,
                        message.message_id,
                        // We ensured the value's existence when took the
                        // amount of iterations
                        *places.next().unwrap(),
                    )
                    .into_future()
                    .map_err(|error| {
                        eprintln!("Whops, an error happened: {:#?}", error);
                    })
                    .map(|_| ())
                })
        });

    tbot::run(request);
}
