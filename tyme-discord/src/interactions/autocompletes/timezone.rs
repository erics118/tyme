use anyhow::{Context as _, Result};
use chrono_tz::TZ_VARIANTS;
use serenity::{
    self,
    builder::{CreateAutocompleteResponse, CreateInteractionResponse},
    client::Context,
    model::application::CommandInteraction,
};

use crate::{get_options, utils::fuzzy_autocomplete::fuzzy_autocomplete};

/// Autocomplete with valid timezones.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let ((cur, _kind),) = get_options!(command, .[Autocomplete]);

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
