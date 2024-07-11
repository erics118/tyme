/// Macro that declares modules for interaction commands and a function to
/// execute interaction commands. This also supports declaring the modules of
/// subcommands
///
/// Format:
///
/// ```
/// interaction_commands!(
///     command1,
///     command2[subcommand1, subcommand2]
///     command3,
/// )   ```
#[macro_export]
macro_rules! interaction_commands {
    // (@name $name:ident) => {
    //     $name
    // };

    // (@name $name1:ident $($name:ident)+) => {
    //     paste::paste! {
    //        [< $name1 _ $crate::interaction_commands!(@name $( $name )+ )  >]
    //     }
    // };

    // (@create_mod $($name:ident)+) => { paste::paste! {
    //     #[doc = concat!(stringify!(crate::interaction_commands!(@name $( $name )+)), " interaction command.")]
    //     pub mod [< $crate::interaction_commands!(@name $( $name )+) >];
    // } };

    (
        $(
            $command:ident
            $(
                [
                    $( $subcmd:ident ),+
                ]
            )?
        ),+
        $(,)?
    ) => { paste::paste!{

        pub use anyhow::Result;
        pub use serenity::{
            client::Context,
            http::Http,
            model::application::{Command, CommandInteraction},
        };

        $(
            #[doc = $command " interaction command."]
            pub mod $command;

            $(
                $(
                    #[doc = concat!(stringify!($command), "_", stringify!($subcmd), " interaction command.")]
                    pub mod [< $command _ $subcmd >];
                )+
            )?
        )+

        /// Function to execute interaction commands.
        pub async fn exec(ctx: Context, command: CommandInteraction) -> Result<()> {
            match command.data.name.as_str() {
                $(
                    stringify!($command) => $crate::interactions::commands::$command::run(ctx, command).await?,
                )+
                _ => (),
            }

            Ok(())
        }

        /// Function to register all interaction commands.
        ///
        /// Adding the Send + Sync traits fixes the clippy::future_not_send diagnostic.
        pub async fn register_all(http: impl AsRef<Http> + Send + Sync) -> Result<()> {
            let _ = Command::set_global_commands(
                http,
                vec![
                    $(
                        $command::register(),
                    )+
                ],
            )
            .await?;
            Ok(())
        }

    } };
}

/// Macro that declares modules for interaction autocompletes and a function to
/// execute interaction autocompletes.
#[macro_export]
macro_rules! interaction_autocompletes {
    ($($cmd:ident),+ $(,)?) => { paste::paste! {
        use anyhow::Result;
        use serenity::{
            client::Context,
            model::application::CommandInteraction,
        };

        $(
            #[doc = $cmd " interaction autocomplete."]
            pub mod $cmd;
        )+

        /// Function to execute interaction autocompletes.
        pub async fn exec(ctx: Context, autocomplete: CommandInteraction) -> Result<()> {
            match autocomplete.data.name.as_str() {
                $(stringify!($cmd) => $crate::interactions::autocompletes::$cmd::run(ctx, autocomplete).await?,)+

                _ => (),
            }

            Ok(())
        }
    } };
}
