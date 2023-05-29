use anyhow::{Context as _, Result};
use serenity::{
    self,
    all::{CommandInteraction, ResolvedValue},
    client::Context,
};
pub async fn run(_ctx: Context, command: CommandInteraction) -> Result<()> {
    let o = command.data.options();

    let subcommand = o.get(0).context("missing option")?;

    let ResolvedValue::SubCommand(ref a) = subcommand.value else { panic!("f")};
    let ResolvedValue::Autocomplete{value: _cur, ..} = &a.get(0)
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
    // todo!()

    Ok(())
}
