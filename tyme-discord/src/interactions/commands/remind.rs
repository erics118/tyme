use anyhow::{Context as _, Result};
use chrono::Utc;
use chrono_tz::Tz;
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    builder::{
        CreateCommand, CreateCommandOption, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    client::Context,
    model::application::CommandOptionType,
};
use tyme_db::{reminders::reminder::Reminder, timezones::timezone::Timezone};
use tyme_utils::human_time::{HumanTime, CheckedAddHumanTime};
use uuid::Uuid;

use crate::data::database::Database;

pub fn register() -> CreateCommand {
    CreateCommand::new("remind")
        .description("Remind you about something")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "when", "When to remind you")
                .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "message",
                "What to remind you about",
            )
            .required(true),
        )
}

pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let ResolvedValue::String(when) = &o.get(0).context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    let ResolvedValue::String(description) = &o.get(1).context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    let data = ctx.data.read().await;

    let pool = data
        .get::<Database>()
        .context("Expected `Database` in TypeMap")?;

    // get user's timezone
    let timezone: Tz = Timezone::get(pool, command.user.id)
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
        id: Uuid::new_v4(),
        created_at: Utc::now().naive_utc(),
        time: now,
        message: description.to_string(),
        user_id: command.user.id,
        channel_id: command.channel_id,
        guild_id: command.guild_id,
    };

    r.create(pool).await?;

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
