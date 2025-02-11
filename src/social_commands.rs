use poise::serenity_prelude::colours::roles::GOLD;
use poise::serenity_prelude::{
    ChannelId, CreateEmbed, CreateEmbedAuthor, CreateMessage, Mentionable,
};
use poise::Modal;

use crate::{Data, Error};

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

/// Share your quiet time in the quiet time channel
#[poise::command(slash_command, aliases("quiet time"))]
pub async fn quiet_time(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = QuietTimeModal::execute(ctx).await?;
    let embed: CreateEmbed = match &data {
        None => return Ok(()),
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
