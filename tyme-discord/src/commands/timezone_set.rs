use tyme_db::{
    chrono_tz::{Tz, TZ_VARIANTS},
    Timezone,
};

use crate::{types::*, utils::fuzzy_autocomplete::fuzzy_autocomplete};

async fn autocomplete_timezone<'a>(_ctx: Context<'_>, partial: &str) -> Vec<String> {
    let values = TZ_VARIANTS.iter().map(|v| v.name()).collect::<Vec<&str>>();

    fuzzy_autocomplete::<&str>(partial, &values)
        .into_iter()
        .map(|s| s.to_string())
        .collect()
}

/// Sets your timezone.
#[poise::command(slash_command)]
pub async fn set(
    ctx: Context<'_>,
    #[description = "Your timezone."]
    #[autocomplete = "autocomplete_timezone"]
    timezone: String,
) -> Result<(), Error> {
    let db = ctx.data().db.lock().await;

    let timezone = match Tz::from_str_insensitive(&timezone) {
        Ok(t) => t,
        Err(_) => {
            ctx.reply("Invalid timezone. You can find them here: <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>")
                .await?;

            return Ok(());
        },
    };

    let t = Timezone {
        user_id: ctx.author().id.into(),
        timezone,
    };

    t.set(&db).await?;

    ctx.reply(format!("Set your timezone to `{}`", t.timezone.name()))
        .await?;

    Ok(())
}
