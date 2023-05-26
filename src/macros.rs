#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),*) => (
        use anyhow::Result;
        use serenity::{client::Context, model::prelude::Message};

        $(pub mod $cmd;)*

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
        use anyhow::Result;
        use serenity::{
            client::Context,
            model::application::CommandInteraction,
        };

        $(pub mod $cmd;)*

        pub async fn exec(ctx: Context, command: CommandInteraction) -> Result<()> {
            match command.data.name.as_str() {
                $(stringify!($cmd) => $crate::interactions::commands::$cmd::run(ctx, command).await?,)*

                #[allow(unreachable_patterns)]
                _ => todo!(),
            }

            Ok(())
        }

        pub async fn register_all(ctx: Context)  -> Result<()> {
            drop(serenity::all::Command::set_global_commands(
                &ctx.http,
                vec![
                    $($crate::interactions::commands::$cmd::register(),)*
                ],
            )
            .await?);
            Ok(())
        }
    );
}
