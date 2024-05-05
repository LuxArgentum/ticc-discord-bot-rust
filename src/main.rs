use std::u32;

use poise::Modal;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{
    ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, Mentionable,
};
use poise::serenity_prelude::colours::roles::GOLD;

#[derive(Debug)]
struct Data {}

// User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, default_member_permissions = "MANAGE_MESSAGES")]
async fn announcement(
    ctx: Context<'_>,
    #[description = "Make an announcement"] description: Option<String>,
) -> Result<(), Error> {
    // TODO: Make description required
    ctx.say(description.unwrap().as_str()).await?;
    Ok(())
}

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
    start_verse: String,
    #[name = "Ending Verse"]
    #[placeholder = "Enter the Book Chapter:Verse here"]
    #[min_length = 5]
    #[max_length = 15]
    end_verse: String,
    #[name = "Summary"]
    #[paragraph]
    #[placeholder = "Optional: Enter a short summary"]
    #[min_length = 5]
    #[max_length = 1024]
    summary: Option<String>,
}

type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

/// TODO: Make Help Text
#[poise::command(slash_command, on_error = "error_handler")]
pub async fn quiet_time(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = QuietTimeModal::execute(ctx).await?;
    let embed: CreateEmbed = match &data {
        None => {
            panic!("Embed was None")
        }
        Some(data) => {
            let author = &ctx.author();
            let mut embed = CreateEmbed::new()
                .author(CreateEmbedAuthor::new(author.global_name.as_ref().unwrap())
                    .icon_url(ctx.author().avatar_url().unwrap()))
                .title(format!("{}'s Quiet Time", ctx.author().global_name.as_ref().unwrap()))
                .color(GOLD)
                .thumbnail("https://cdn.dribbble.com/users/113758/screenshots/4257859/media/47f55de9a2ddd003143b2dc792a12f1e.jpg?resize=400x300&vertical=center")
                .field(
                    "Verses",
                    format!("From **{}** to **{}**", data.start_verse, data.end_verse),
                    false);
            if let Some(summary) = &data.summary {
                embed = embed.field("Summary", summary, false);
            }
            embed
        }
    };
    let quiet_time_channel_id =
        std::env::var("QUIET_TIME_CHANNEL_ID").expect("missing QUIET_TIME_CHANNEL_ID");
    let quiet_time_channel = ChannelId::new(quiet_time_channel_id.parse::<u64>().unwrap());
    quiet_time_channel
        .send_message(ctx, CreateMessage::new().embed(embed))
        .await?;
    ctx.reply(format!(
        "Hey {user}! Your quiet time has been shared!",
        user = ctx.author().mention()
    ))
    .await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn enter_birthday(
    ctx: Context<'_>,
    #[description = "Birth month"] month: String,
    #[description = "Birth day"] day: u32,
) -> Result<(), Error> {
    let user = ctx.author();
    let response = format!("{}'s birthday is {} {}", user.name, month, day);
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
    println!("Oh noes, we got an error: {error:?}");
}

enum Testing {
    Testing(serenity::GuildId),
    NotTesting,
}

#[tokio::main]
async fn main() {
    run_server().await;
}

async fn run_server() {

    dotenv::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

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
            commands: vec![
                age(),
                enter_birthday(),
                quiet_time(),
                register(),
                announcement(),
            ],
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
