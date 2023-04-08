#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),*) => (
        impl MessageCommands {
            async fn exec(command: &str, ctx: Context, message: Message) -> Result<()> {
                match command {
                    $(stringify!($cmd) => $crate::messages::commands::$cmd::run(ctx, message).await?,)*

                    #[allow(unreachable_patterns)]
                    &_ => todo!(),
                }
                Ok(())
            }
        }
    );
}

#[macro_export]
macro_rules! interaction_commands {
    ($($cmd:ident),*) => (
        impl InteractionCommands {
            async fn exec(ctx: Context, command: ApplicationCommandInteraction) -> Result<()>{
                match command.data.name.as_str() {
                    $(stringify!($cmd) => $crate::interactions::commands::$cmd::run(ctx, command).await?,)*

                    #[allow(unreachable_patterns)]
                    _ => todo!(),
                }Ok(
()
                    )
            }
        }
    );
}
