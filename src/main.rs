use std::u32;

use poise::serenity_prelude as serenity;

struct Data {}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn get_birthday(
    ctx: Context<'_>,
    #[description = "Birth month"] month: String,
    #[description = "Birth day"] day: u32,
) -> Result<(), Error> {
    let user = ctx.author();
    let month = month;
    let day = day;
    let response = format!("{}'s birthday is {} {}", user.name, month, day);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    enum Testing {
        Testing(serenity::GuildId),
        NotTesting,
    }
    let testing = match std::env::var("GUILD_ID") {
        Ok(id) => {
            let id: u64 = id.parse().expect("not a 64-bit unsigned integer");
            Testing::Testing(id.into())
        }
        Err(_) => Testing::NotTesting,
    };
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), get_birthday()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                match testing {
                    Testing::Testing(guild_id) => {
                        poise::builtins::register_in_guild(
                            ctx, &framework.options().commands, guild_id,
                        ).await?;
                    }
                    Testing::NotTesting => {
                        poise::builtins::register_globally(
                            ctx, &framework.options().commands,
                        ).await?;
                    }
                }
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}