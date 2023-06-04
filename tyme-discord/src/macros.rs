#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),*) => (
        use anyhow::Result;
        use serenity::{client::Context, model::prelude::Message};

        $(pub mod $cmd;)*

        pub async fn exec(command: &str, ctx: Context, message: Message) -> Result<()> {
            match command {
                $(stringify!($cmd) => $crate::messages::commands::$cmd::run(ctx, message).await?,)*

                #[allow(unreachable_patterns)]
                _ => (),
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
            http::Http,
            model::application::{Command, CommandInteraction},
        };

        $(pub mod $cmd;)*

        pub async fn exec(ctx: Context, command: CommandInteraction) -> Result<()> {
            match command.data.name.as_str() {
                $(stringify!($cmd) => $crate::interactions::commands::$cmd::run(ctx, command).await?,)*

                #[allow(unreachable_patterns)]
                _ => (),
            }

            Ok(())
        }

        // adding the + Send + Sync fixes the clippy::future_not_send diagnostic
        pub async fn register_all(http: impl AsRef<Http> + Send + Sync)  -> Result<()> {
            let _ = Command::set_global_commands(
                http,
                vec![
                    $($crate::interactions::commands::$cmd::register(),)*
                ],
            )
            .await?;
            Ok(())
        }
    );
}

#[macro_export]
macro_rules! interaction_autocompletes {
    ($($cmd:ident),*) => (
        use anyhow::Result;
        use serenity::{
            client::Context,
            model::application::CommandInteraction,
        };

        $(pub mod $cmd;)*

        pub async fn exec(ctx: Context, autocomplete: CommandInteraction) -> Result<()> {
            match autocomplete.data.name.as_str() {
                $(stringify!($cmd) => $crate::interactions::autocompletes::$cmd::run(ctx, autocomplete).await?,)*

                #[allow(unreachable_patterns)]
                _ => (),
            }

            Ok(())
        }
    );
}
