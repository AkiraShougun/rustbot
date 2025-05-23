use poise::serenity_prelude::all::ChannelId;
use poise::serenity_prelude::{CreateMessage, Http};
use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};
use std::{env, io};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

//Register
#[poise::command(slash_command, prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    let commands = vec![ ping(),
                lmfdb(),
                secret(),
                secret2(),register()];
    poise::builtins::register_in_guild(ctx, &commands, ctx.guild_id().unwrap()).await?;
    Ok(())
}


// Basic commands
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn secret(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Let X be an algebraic projective variety").await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn secret2(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("omg i love modular forms").await?;
    Ok(())
}

// LMFDB command group
#[poise::command(slash_command, prefix_command, subcommands("modular_forms","number_fields"))]
pub async fn lmfdb(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Welcome to LMFDB commands! Try `/lmfdb modular_forms` for modular forms functionality.").await?;
    Ok(())
}

// Modular forms subcommand group
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("mf_properties"),
    subcommand_required
)]
pub async fn modular_forms(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Modular forms commands - please use a subcommand").await?;
    Ok(())
}

//Number fields subcommand group
#[poise::command(
    slash_command,
    prefix_command,
    subcommands("nf_properties"),
    subcommand_required
)]
pub async fn number_fields(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Number fields commands - please use a subcommand").await?;
    Ok(())
}


// Properties subcommand
#[poise::command(slash_command, prefix_command)]
pub async fn mf_properties(
    ctx: Context<'_>,
    #[description = "Level"] level: Option<i32>,
    #[description = "Weight"] weight: Option<i32>,
    #[description = "Character"] character: Option<String>,
) -> Result<(), Error> {
    let response = format!(
        "Modular form properties - Level: {:?}, Weight: {:?}, Character: {:?}",
        level, weight, character
    );
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn nf_properties(
    ctx: Context<'_>,
    #[description = "Discriminant"] discriminant: Option<i32>,
) -> Result<(), Error> {
    let response = format!(
        "Number fields properties - discriminant: {:?}",
        discriminant
    );
    ctx.say(response).await?;
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
            commands: vec![
                ping(),
                lmfdb(),
                secret(),
                secret2(),register()
            ],
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