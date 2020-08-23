use tbot::{types::InteriorBorrow, Bot};

#[tokio::main]
async fn main() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.document(|context| async move {
        let call_result = context
            .bot
            .get_file(context.document.file_id.borrow_inside())
            .call()
            .await;
        let file = match call_result {
            Ok(file) => file,
            Err(err) => {
                dbg!(err);
                return;
            }
        };

        let call_result = context.bot.download_file(&file).await;
        let bytes = match call_result {
            Ok(bytes) => bytes,
            Err(err) => {
                dbg!(err);
                return;
            }
        };

        match String::from_utf8(bytes) {
            Ok(document) => println!("{}", document),
            Err(err) => {
                dbg!(err);
            }
        }
    });

    bot.polling().start().await.unwrap();
}
