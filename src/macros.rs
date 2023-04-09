#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),*) => (
        pub async fn exec(command: String, ctx: Context, message: Message) -> Result<()> {
            match command.as_str() {
                $(stringify!($cmd) => $crate::messages::commands::$cmd::run(ctx, message).await?,)*

                #[allow(unreachable_patterns)]
                _ => todo!(),
            }
            Ok(())
        }
    );
}

#[macro_export]
macro_rules! interaction_commands {
    ($($cmd:ident),*) => (
        pub async fn exec(ctx: Context, command: ApplicationCommandInteraction) -> Result<()>{
            match command.data.name.as_str() {
                $(stringify!($cmd) => $crate::interactions::commands::$cmd::run(ctx, command).await?,)*

                #[allow(unreachable_patterns)]
                _ => todo!(),
            }
            Ok(())
        }

        pub fn register_all(commands: &mut CreateApplicationCommands) -> &mut CreateApplicationCommands {
            commands
                $(.create_application_command(|command| $crate::interactions::commands::$cmd::register(command)))*

            // $($crate::interactions::commands::$cmd::register(command);)*
                // command
        }
    );
}
