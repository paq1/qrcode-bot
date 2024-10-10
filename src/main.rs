use qrcode_bot::run_discord_bot;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    run_discord_bot(&token).await;
}
