#[tokio::main]
async fn main() {
    let bot = tbot::from_env!("BOT_TOKEN");
    let me = bot.get_me().call().await.unwrap();
    dbg!(me);
}
