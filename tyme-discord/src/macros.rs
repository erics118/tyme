//! Utility macros.

/// Macro that declares modules for each message command and a function to
/// execute each message command.
#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),+ $(,)?) => (
        use anyhow::Result;
        use serenity::{client::Context, model::prelude::Message};

        $(
            #[doc = concat!(stringify!($cmd), " message command.")]
            pub mod $cmd;
        )+

        /// Function to execute message commands.
        pub async fn exec(command: &str, ctx: Context, message: Message) -> Result<()> {
            match command {
                $(stringify!($cmd) => $crate::messages::commands::$cmd::run(ctx, message).await?,)+

                _ => (),
            }

            Ok(())
        }
    );
}

/// Macro that declares modules for interaction commands and a function to
/// execute interaction commands. This also supports declaring the modules of
/// subcommands
#[macro_export]
macro_rules! interaction_commands {
    ($($command:ident$([$( $subcommand:ident ),*])?),+ $(,)?) => { paste::paste!{

        pub use anyhow::Result;
        pub use serenity::{
            client::Context,
            http::Http,
            model::application::{Command, CommandInteraction},
        };

        $(
            #[doc = concat!(stringify!($command), " interaction command.")]
            pub mod $command;

            $(
                $(
                    #[doc = concat!(stringify!($command), "_", stringify!($subcommand), " interaction command.")]
                    pub mod [<$command _ $subcommand>];
                )*
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
        pub async fn register_all(http: impl AsRef<Http> + Send + Sync)  -> Result<()> {
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
    ($($cmd:ident),+ $(,)?) => (
        use anyhow::Result;
        use serenity::{
            client::Context,
            model::application::CommandInteraction,
        };

        $(
            #[doc = concat!(stringify!($cmd), " interaction autocomplete.")]
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
    );
}

/// Create an option
#[macro_export]
macro_rules! create_option {
    (
        $option_type:ident $option_name:ident $option_description:literal $($option_required:literal)?
        $(>> $sub_option_type:ident $sub_option_name:ident $sub_option_description:literal $($sub_option_required:literal)?)*
    ) => {
        paste::paste! {
            CreateCommandOption::new(
                CommandOptionType::$option_type,
                stringify!($option_name),
                $option_description,
            )
            $(.required($option_required))?
            $(
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::$sub_option_type,
                        stringify!($sub_option_name),
                        $sub_option_description,
                    )
                    $(.required($sub_option_required),)?
                )
            )*

        }
    };
}

/// Create interaction command macro
#[macro_export]
macro_rules! create_interaction_command {
    (
        $name:ident
        | $description:literal
        $(
            > $option_type:ident $option_name:ident $option_description:literal $option_required:literal
            $(>> $sub_option_type:ident $sub_option_name:ident $sub_option_description:literal $sub_option_required:literal)*
        )*
    ) => {
        paste::paste! {
            #[allow(unused)]
            use serenity::{
                builder::{CreateCommand, CreateCommandOption},
                model::application::CommandOptionType,
            };

            #[doc = concat!("create the ", stringify!($name), " interaction command.")]
            #[allow(unused)]
            pub fn register() -> CreateCommand {
                let mut c = CreateCommand::new(stringify!($name));

                c = c.description($description);

                $(
                    c = c.add_option($crate::create_option!(
                        $option_type $option_name $option_description $option_required
                        $(>> $sub_option_type $sub_option_name $sub_option_description $sub_option_required)*
                    ));
                )*

                return c;
            }
        }
    };
}

/// Create interaction with only subcommands
#[macro_export]
macro_rules! create_interaction_command_only_subcommands {
    (
        $name:tt
        $(
            + $option_name:ident $option_description:literal
            $(>> $sub_option_type:ident $sub_option_name:ident $sub_option_description:literal $sub_option_required:literal)*
        )+
    ) => {
        paste::paste! {
            #[allow(unused)]
            use anyhow::{Context as _, Result};
            #[allow(unused)]
            use serenity::{
                all::CommandInteraction,
                builder::{CreateCommand, CreateCommandOption},
                client::Context,
                model::application::CommandOptionType,
            };

            #[doc = concat!("create the ", stringify!($name), " interaction command.")]
            #[allow(unused)]
            pub fn register() -> CreateCommand {
                let mut c = CreateCommand::new(stringify!($name))
                    .description("*");

                $(
                    c = c.add_option($crate::create_option!(
                        SubCommand $option_name $option_description
                        $(>> $sub_option_type $sub_option_name $sub_option_description $sub_option_required)*
                    ));
                )+

                return c;
            }

            /// Handle the timezone command.
            pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
                let o = command.data.options();

                let subcommand = &o.get(0).context("missing option")?;

                match subcommand.name {
                    $(
                        stringify!($option_name) => super::[<$name _ $option_name>]::run(ctx, command).await?,
                    )+
                    _ => unreachable!(),
                };

                Ok(())
            }


        }
    };
}
