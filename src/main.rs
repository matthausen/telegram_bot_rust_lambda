use lambda_http::{run, service_fn, tracing, Body, Error, Request};
use teloxide::types::Message;
use teloxide::types::UpdateKind;
use teloxide::utils::command::BotCommands;
use teloxide::prelude::*;
use std::env;
use tracing::{error, warn};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum BotCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "welcome message.")]
    Start,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    // Initialise the bot
    let bot = Bot::new(env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set!"));
    // Set commands
    let res = bot.set_my_commands(BotCommand::bot_commands()).await;
    if let Err(e) = res {
        warn!("Failed to set commands: {:?}", e);
    }

    // Run the Lambda function
    run(service_fn(|req| handler(req, &bot))).await
}

async fn handler(
    req: lambda_http::Request,
    bot: &Bot,
) -> Result<lambda_http::Response<String>, lambda_http::Error> {
    // Parse JSON webhook
    let bot = bot.clone();

    let update = match parse_webhook(req).await {
        Ok(message) => message,
        Err(e) => {
            error!("Failed to parse webhook: {:?}", e);
            return Ok(lambda_http::Response::builder()
                .status(400)
                .body("Failed to parse webhook".into())
                .unwrap());
        }
    };

    // Handle commands
    if let UpdateKind::Message(message) = &update.kind {
        if let Some(text) = &message.text() {
            if let Ok(command) = BotCommand::parse(text, bot.get_me().await.unwrap().username()) {
                return handle_command(bot.clone(), message, command).await;
            }
        }
    }

    Ok(lambda_http::Response::builder()
    .status(200)
    .body(String::new())
    .unwrap())

}

async fn handle_command(
    bot: Bot,
    message: &Message,
    command: BotCommand,
) -> Result<lambda_http::Response<String>, lambda_http::Error> {
    match command {
        BotCommand::Help => {
            bot.send_message(message.chat.id, BotCommand::descriptions().to_string())
                .await
                .unwrap();
        }
        BotCommand::Start => {
            bot.send_message(message.chat.id, "Welcome! Send a voice message or video note to transcribe it. You can also use /help to see all available commands. Currently there are no other commands available.")
                .await
                .unwrap();
        }
    }

    Ok(lambda_http::Response::builder()
        .status(200)
        .body(String::new())
        .unwrap())
}

pub async fn parse_webhook(input: Request) -> Result<Update, Error> {
    let body = input.body();
    let body_str = match body {
        Body::Text(text) => text,
        not => panic!("expected Body::Text(...) got {not:?}"),
    };
    let body_json: Update = serde_json::from_str(body_str)?;
    Ok(body_json)
}
