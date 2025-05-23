use poise::{PrefixFrameworkOptions, serenity_prelude as serenity};
use std::env;

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
