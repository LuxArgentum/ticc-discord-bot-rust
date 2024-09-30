use std::panic::panic_any;

use crate::{Context, Error};

#[poise::command(
    slash_command,
    default_member_permissions = "MANAGE_MESSAGES",
    aliases("Announcement")
)]
pub async fn announcement(
    ctx: Context<'_>,
    #[description = "Make an announcement"] message: Option<String>,
) -> Result<(), Error> {
    // TODO: Make description required
    let message: String = match message {
        None => panic_any("Announcement message is required"),
        Some(string) => string,
    };
    ctx.say(message).await?;
    Ok(())
}

/// Register the application commands
#[poise::command(slash_command, owners_only)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
