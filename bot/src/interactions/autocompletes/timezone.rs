use anyhow::{Context as _, Result};
use serenity::{
    all::{CommandInteraction, ResolvedValue},
    builder::{
        CreateAutocompleteResponse, CreateInteractionResponse, CreateInteractionResponseMessage,
    },
    client::Context,
};
pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};
    let ResolvedValue::Autocomplete{value: cur, ..} = &a.get(0)
        .context("missing option")?.value else {
        anyhow::bail!("incorrect resolved option type")
    };

    // command
    //     .create_response(
    //         &ctx.http,
    //         CreateInteractionResponse::Autocomplete(
    //             CreateAutocompleteResponse::new().set_choices(),
    //         ),
    //     )
    //     .await?;
    todo!()
}