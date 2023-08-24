use anyhow::{Context as _, Result};
use chrono::Utc;
use chrono_tz::Tz;
use serenity::{
    all::CommandInteraction,
    builder::{CreateInteractionResponse, CreateInteractionResponseMessage},
    client::Context,
};
use tyme_db::{Reminder, Timezone};

use crate::{
    create_command,
    data::database::Database,
    get_options,
    utils::human_time::{CheckedAddHumanTime, HumanTime},
};

create_command! {
    / remind
    | "Remind you about something"
    > String when "When to remind you" required
    > String message "What to remind you about" required
}

/// Handle the remind command.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let (when, description) = get_options!(command, String, String);

    let db = {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .context("Expected `Database` in TypeMap")?
            .clone()
    };

    // get user's timezone
    let timezone: Tz = Timezone::get(&db, command.user.id)
        .await
        .map_or_else(|_| Tz::UTC, |t| t.timezone);

    let now = Utc::now().naive_utc();

    // parse `when`
    let Ok(a) = HumanTime::parse(when) else {
        command
            .create_response(
                &ctx.http,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content("Invalid time."),
                ),
            )
            .await?;

        return Ok(());
    };

    let Ok(now)  = now.checked_add(a) else {
        command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content("Invalid time."),
            ),
        )
        .await?;

        return Ok(());
    };

    // create reminder
    let r = Reminder {
        id: None,
        created_at: Utc::now().naive_utc(),
        time: now,
        message: description.to_string(),
        user_id: command.user.id,
        channel_id: command.channel_id,
        guild_id: command.guild_id,
    };

    r.create(&db).await?;

    let msg = format!(
        "Reminder set for {} on **{}**",
        r.message,
        r.time
            .and_utc()
            .with_timezone(&timezone)
            .format("%h %e, %Y at %l:%M %p %Z"),
    );

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().content(msg),
            ),
        )
        .await?;

    Ok(())
}
