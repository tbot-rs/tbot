use tbot::Bot;

#[tokio::main]
async fn main() -> Result<(), tbot::errors::MethodCall> {
    let bot = Bot::from_env("BOT_TOKEN");

    let me = bot.get_me().call().await?;
    dbg!(me);

    Ok(())
}
