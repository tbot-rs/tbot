use std::sync::Arc;

#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.document(|context| {
        let bot = Arc::clone(&context.bot);
        let document = context.document.clone();

        tokio::spawn(async move {
            let file = bot.get_file(&document).call().await.unwrap();

            let bytes = bot.download_file(&file).await.unwrap();
            match String::from_utf8(bytes) {
                Ok(document) => println!("{}", document),
                Err(err) => {
                    dbg!(err);
                }
            }
        });
    });

    bot.polling().start().await.unwrap();
}
