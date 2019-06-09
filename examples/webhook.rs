use tbot::{prelude::*, types::ParseMode::Markdown};

const URL: &str = "https://example.com";
const PORT: u16 = 2000;

fn main() {
    let mut bot = tbot::bot!("BOT_TOKEN");

    bot.text(|context| {
        let message = match meval::eval_str(&context.text.value) {
            Ok(result) => format!("= `{}`", result),
            Err(_) => "Whops, I couldn't evaluate your expression :(".into(),
        };

        let reply = context
            .send_message_in_reply(&message)
            .parse_mode(Markdown)
            .into_future()
            .map_err(|error| {
                dbg!(error);
            });

        tbot::spawn(reply);
    });

    bot.webhook(URL, PORT).start();
}
