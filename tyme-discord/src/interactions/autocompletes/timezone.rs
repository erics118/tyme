use anyhow::{Context as _, Result};
use chrono_tz::TZ_VARIANTS;
use serenity::{
    self,
    all::{CommandInteraction, ResolvedValue},
    builder::{CreateAutocompleteResponse, CreateInteractionResponse},
    client::Context,
};

use crate::utils::fuzzy_autocomplete::fuzzy_autocomplete;

/// Autocomplete with valid timezones.
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};
    let ResolvedValue::Autocomplete{value: cur, ..} = &a.get(0)
        .context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    let values = TZ_VARIANTS.iter().map(|v| v.name()).collect::<Vec<_>>();

    let choices = fuzzy_autocomplete(cur, &values).context("autocomplete failed")?;

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
