//! Utility macros.

/// Macro that declares modules for each message command and a function to
/// execute each message command.
#[macro_export]
macro_rules! message_commands {
    ($($cmd:ident),+ $(,)?) => { paste::paste! {
        use anyhow::Result;
        use serenity::{client::Context, model::prelude::Message};

        $(
            #[doc = $cmd " message command."]
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
    } };
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
            #[doc = $command " interaction command."]
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

/// Create an extra basic option
///
/// Format:
/// ```
/// create_extra_basic_option!(String name "Description")
/// ```
#[macro_export]
macro_rules! create_extra_basic_option {
    (
        $option_type:ident $option_name:ident $option_description:literal
    ) => {
        CreateCommandOption::new(
            CommandOptionType::$option_type,
            stringify!($option_name),
            $option_description,
        )
    };
}

/// Create a basic option
///
/// Format:
/// ```
/// create_basic_option!(String name "Description" [optional/required] [autocomplete])
/// ```
#[macro_export]
macro_rules! create_basic_option {
    ($option_type:tt $option_name:ident $option_description:literal) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
    };
    ($option_type:ident $option_name:ident $option_description:literal optional) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
            .required(false)
    };
    ($option_type:ident $option_name:ident $option_description:literal required) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
            .required(true)
    };
    ($option_type:ident $option_name:ident $option_description:literal autocomplete) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
            .set_autocomplete(true)
    };
    ($option_type:ident $option_name:ident $option_description:literal optional autocomplete) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
            .required(false)
            .set_autocomplete(true)
    };
    ($option_type:ident $option_name:ident $option_description:literal required autocomplete) => {
        $crate::create_extra_basic_option!($option_type $option_name $option_description)
            .required(true)
            .set_autocomplete(true)
    };
}

/// Create an option
///
/// Format:
///
/// ```
/// create_option!(
///     String name "Description" [optional/required] [autocomplete])
///     >> String another "Description" [optional/required] [autocomplete]
/// ```
#[macro_export]
macro_rules! create_option {
    (
        $option_type:ident $option_name:ident $option_description:literal $($option_other:ident)*
        $(
            >> $suboption_type:ident $suboption_name:ident $suboption_description:literal $($suboption_other:ident)*
        )*
    ) => {
        $crate::create_basic_option!($option_type $option_name $option_description $($option_other)*)
        $(
            .add_sub_option(
                $crate::create_basic_option!($suboption_type $suboption_name $suboption_description $($suboption_other)*)
            )
        )*
    };
}

/// Create interaction command macro
///
/// Format:
///
/// ```
/// create_interaction_command!(
///    name
///   | "Description"
///    > String option_name "Description" [optional/required] [autocomplete]
///    >> String suboption_name "Description" [optional/required] [autocomplete]
///    > String another_option_name "Description" [optional/required] [autocomplete]
/// )
/// ```
#[macro_export]
macro_rules! create_interaction_command_no_subcommands {
    (
        $name:ident
        | $description:literal
        $(
            > $option_type:ident $option_name:ident $option_description:literal $($option_other:ident)*
            $(
                >> $suboption_type:ident $suboption_name:ident $suboption_description:literal $($suboption_other:ident)*
            )*
        )*
    ) => { paste::paste! {
        #[allow(unused)]
        use serenity::{
            builder::{CreateCommand, CreateCommandOption},
            model::application::CommandOptionType,
        };

        #[doc = "create the " $name " interaction command."]
        pub fn register() -> CreateCommand {
            let mut c = CreateCommand::new(stringify!($name));

            c = c.description($description);

            $(
                c = c.add_option(
                    $crate::create_option!(
                        $option_type $option_name $option_description $($option_other)*
                        $(>> $suboption_type $suboption_name $suboption_description $($suboption_other)*)*
                    )
                );
            )*

            return c;
        }
    } };
}

/// Create interaction command with only subcommands
///
/// Format:
///
/// ```
/// create_interaction_command_only_subcommands!(
///     name
///     + option_name "Description"
///     >> String suboption_name "Description"
///     + another_option_name "Description"
/// )
#[macro_export]
macro_rules! create_interaction_command_only_subcommands {
    (
        $name:ident
        $(
            + $option_name:ident $option_description:literal
            $(
                >> $suboption_type:ident $suboption_name:ident $suboption_description:literal $($suboption_other:ident)*
            )*
        )+
    ) => { paste::paste! {

        #[allow(unused)]
        use anyhow::{Context as _, Result};
        #[allow(unused)]
        use serenity::{
            all::CommandInteraction,
            builder::{CreateCommand, CreateCommandOption},
            client::Context,
            model::application::CommandOptionType,
        };

        #[doc = "create the " $name " interaction command."]
        #[allow(unused)]
        pub fn register() -> CreateCommand {
            let mut c = CreateCommand::new(stringify!($name))
                .description("*");

            $(
                c = c.add_option(
                    $crate::create_option!(
                        SubCommand $option_name $option_description
                        $(>> $suboption_type $suboption_name $suboption_description $($suboption_other)*)*
                    )
                );
            )+

            return c;
        }

        /// Handle the timezone command.
        pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
            let o = command.data.options();

            let subcommand = &o.get(0).context("missing option")?;

            match subcommand.name {
                $(
                    stringify!($option_name) => super::[< $name _ $option_name >]::run(ctx, command).await?,
                )+
                _ => unreachable!(),
            };

            Ok(())
        }

    } };
}

/// Create interaction command macro
///
/// Format:
///
/// ```
/// create_interaction_command!(
///     / name
///     | "Description"
///    > String option_name "Description" [optional/required] [autocomplete]
///    >> String suboption_name "Description" [optional/required] [autocomplete]
///    > String another_option_name "Description" [optional/required] [autocomplete]
/// )
///
/// create_interaction_command!(
///     / name
///     + subcmd "Description"
///     >> String option_name "Description" [optional/required] [autocomplete]
///     + another_subcmd "Description"
/// )
/// ```
#[macro_export]
macro_rules! create_command {
    ( / $name:ident | $($other:tt)+ ) => {
        $crate::create_interaction_command_no_subcommands!($name | $($other)+);
    };
    ( / $name:ident + $($other:tt)+ ) => {
        $crate::create_interaction_command_only_subcommands!($name + $($other)+);
    }
}

/// Create interaction response message
#[macro_export]
macro_rules! create_interaction_response_message {
    (
        $($content:expr)? ,
        $( $( # $embed:tt )+ )?
    ) => {
        serenity::builder::CreateInteractionResponse::Message(
            serenity::builder::CreateInteractionResponseMessage::new()
                $( .content($content) )?
                $( $( .add_embed($embed) )+ )?
        )
    };
}

/// Create normal message
#[macro_export]
macro_rules! create_normal_message {
    (
        $($content:expr)? ,
        $( $( # $embed:tt )+ )?
        $( @ $reference_message:tt )?
    ) => {
        serenity::builder::CreateMessage::new()
            $( .content($content) )?
            $( $( .add_embed($embed) )+ )?
            $( .reference_message(&$reference_message) )?
    };
}

/// Create a message
#[macro_export]
macro_rules! create_message {
    ( / $($other:tt)* ) => {
        $crate::create_interaction_response_message!($($other)*)
    };
    ( $($other:tt)* ) => {
        $crate::create_normal_message!($($other)*)
    };
}
