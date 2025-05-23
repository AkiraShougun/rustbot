use poise::serenity_prelude::all::ChannelId;
use poise::serenity_prelude::{CreateMessage, Http};
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};
use std::{env, io};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Pong!";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn secret(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Let X be an algebraic projective variety";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn secret2(ctx: Context<'_>) -> Result<(), Error> {
    let response = "omg i love modular forms";
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn lmfdb(
    ctx: Context<'_>,
    #[description = "Description here"] text: String,
) -> Result<(), Error> {
    ctx.say(text).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let token_clone = token.clone();
    let _terminal_handle = tokio::spawn(async move {
        terminal_tools(&token_clone).await;
    });
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("rusty".into()),
                ..Default::default()
            },
            commands: vec![ping(), lmfdb(), secret(), secret2()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn terminal_tools(token: &String) {
    const TERMINAL_MESSAGE: &str = r"Select an option:
        1) Send a mesasge
        2) DM a user
        3) View History
        4) Ban";
    let http = Http::new(token);
    let mut key = String::new();
    loop {
        println!("{TERMINAL_MESSAGE}");

        io::stdin()
            .read_line(&mut key)
            .expect("failed to read line");

        match key.trim().chars().collect::<Vec<char>>()[0].to_ascii_uppercase() {
            '1' => {
                println!("What server would you like to send the message in (Channel ID): ");
                let mut id_buf = String::new();
                let mut message_buf = String::new();

                io::stdin()
                    .read_line(&mut id_buf)
                    .expect("failed to read line");

                println!("What would you like to say: ");

                io::stdin()
                    .read_line(&mut message_buf)
                    .expect("failed to read line");

                let channel_id = ChannelId::new(
                    id_buf
                        .trim()
                        .parse::<u64>()
                        .expect("That was not a valid channel ID chat"),
                );
                let message = CreateMessage::new().content(message_buf).tts(false);
                let _ = channel_id.send_message(&http, message).await;
            }
            '2' => {}
            '3' => {}
            '4' => {}
            _ => {
                println!("That isn't a 1, 2, 3, or 4 pookie")
            }
        }
    }
}
