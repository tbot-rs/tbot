#[tbot::main]
async fn main() {
    let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();

    bot.document(|context| {
        async move {
            let file = context
                .bot
                .get_file(&context.document)
                .call()
                .await
                .unwrap();

            let bytes = context.bot.download_file(&file).await.unwrap();
            match String::from_utf8(bytes) {
                Ok(document) => println!("{}", document),
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    });

    bot.polling().start().await.unwrap();
}
