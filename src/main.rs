use std::u32;

use poise::Modal;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedAuthor, Mentionable};

#[derive(Debug)]
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

#[derive(Debug, Modal)]
#[name = "Quiet Time Form"]
struct QuietTimeModal {
    #[name = "Starting Verse"]
    #[placeholder = "Enter the Book Chapter:Verse here"]
    #[min_length = 5]
    #[max_length = 15]
    first_input: String,
    #[name = "Ending Verse"]
    #[placeholder = "Enter the Book Chapter:Verse here"]
    #[min_length = 5]
    #[max_length = 15]
    second_input: String,
    #[name = "Summary"]
    #[paragraph]
    #[placeholder = "Optional: Enter a short summary"]
    #[min_length = 5]
    #[max_length = 1000]
    third_input: Option<String>,
}

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

/// TODO: Make Help Text
#[poise::command(
slash_command,
on_error = "error_handler",
)]
pub async fn quiet_time(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = QuietTimeModal::execute(ctx).await?;
    let embed: CreateEmbed = match &data {
        None => { panic!("Embed was None") }
        Some(data) => {
            let author = &ctx.author();
            let mut embed = CreateEmbed::new()
                .author(CreateEmbedAuthor::new(author.global_name.as_ref().unwrap())
                    .icon_url(ctx.author().avatar_url().unwrap()))
                .title(format!("{}'s Quiet Time", ctx.author().global_name.as_ref().unwrap()))
                .field(
                    "Verses",
                    format!("From {} to {}", data.first_input, data.second_input),
                    false);
            if let Some(summary) = &data.third_input {
                embed = embed.field(
                    "Summary",
                    summary,
                    false);
            }
            embed
        }
    };
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    ctx.reply(format!("Hey {user}! Your quiet time has been shared!",
                      user = ctx.author().mention())).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn get_birthday(
    ctx: Context<'_>,
    #[description = "Birth month"] month: String,
    #[description = "Birth day"] day: u32,
) -> Result<(), Error> {
    let user = ctx.author();
    let response = format!("{}'s birthday is {} {}", user.name, month, day);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(
slash_command,
owners_only,
)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Oh noes, we got an error: {:?}", error);
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
            commands: vec![age(), get_birthday(), quiet_time(), register()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                match testing {
                    Testing::Testing(guild_id) => {
                        poise::builtins::register_in_guild(
                            ctx,
                            &framework.options().commands,
                            guild_id,
                        )
                            .await?;
                    }
                    Testing::NotTesting => {}
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
