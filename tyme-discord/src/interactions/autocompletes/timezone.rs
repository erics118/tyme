use anyhow::{Context as _, Result};
use chrono_tz::TZ_VARIANTS;
use serenity::{
    self,
    all::CommandInteraction,
    builder::{CreateAutocompleteResponse, CreateInteractionResponse},
    client::Context,
};

use crate::{get_options, utils::fuzzy_autocomplete::fuzzy_autocomplete};

/// Autocomplete with valid timezones.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let (cur,) = get_options!(command, .Autocomplete);

    let values = TZ_VARIANTS.iter().map(|v| v.name()).collect::<Vec<_>>();

    let choices = fuzzy_autocomplete(cur, &values).context("fuzzy failed")?;

    command
        .create_response(
            &ctx.http,
            CreateInteractionResponse::Autocomplete(
                CreateAutocompleteResponse::new().set_choices(choices),
            ),
        )
        .await?;

    Ok(())
}
