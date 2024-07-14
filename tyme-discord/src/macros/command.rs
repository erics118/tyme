/// Create an option
///
/// Format:
///
/// ```
/// create_option!(
///     String name "Description" [attrs]
/// )
///
/// create_option!(
///     subcmd "Description"
///         > String suboption "Description" [attrs]
/// )
///
/// create_option!(
///     subcmdgroup
///         + subcmd "Description"
///             > String suboption "Description" [attrs]
/// )
/// ```
#[macro_export]
macro_rules! create_option {
    // Helper macro to add attributes to a simple option
    (@add_attrs $opt:expr, ) => { $opt };

    (@add_attrs $opt:expr, optional $($rest:tt)*) => {
        $crate::create_option!(@add_attrs $opt.required(false), $($rest)*)
    };

    (@add_attrs $opt:expr, required $($rest:tt)*) => {
        $crate::create_option!(@add_attrs $opt.required(true), $($rest)*)
    };

    (@add_attrs $opt:expr, autocomplete $($rest:tt)*) => {
        $crate::create_option!(@add_attrs $opt.set_autocomplete(true), $($rest)*)
    };

    // (@add_attrs $opt:expr, min_length($min_length:tt) $($rest:tt)*) => {
    //     $crate::create_option!(@add_attrs $opt.min_length($min_length), $($rest)*)
    // };

    // (@add_attrs $opt:expr, max_length $max_length:tt $($rest:tt)*) => {
    //     $crate::create_option!(@add_attrs $opt.max_length($max_length), $($rest)*)
    // };

    // Simple option without suboptions
    (@simple_option $option_type:ident $option_name:ident $option_description:literal $($attrs:tt)*) => {
        {
            let opt = CreateCommandOption::new(
                CommandOptionType::$option_type,
                stringify!($option_name),
                $option_description,
            );
            $crate::create_option!(@add_attrs opt, $($attrs)*)
        }
    };

    // command with subcmds
    (
        $subcmd_name:ident $subcmd_description:literal
        $(
            > $option_type:ident $option_name:ident $option_description:literal $($option_attrs:ident)*
        )*
    ) => {
        $crate::create_option!(@simple_option SubCommand $subcmd_name $subcmd_description)
        $(
            .add_sub_option(
                $crate::create_option!(@simple_option $option_type $option_name $option_description $($option_attrs)*)
            )
        )*
    };

    // command with group and subcmds
    // subcmds are required here, so using + to repeat rather than *
    (
        $group_name:ident
        $(
            + $subcmd_name:ident $subcmd_description:literal
            $(
                > $option_type:ident $option_name:ident $option_description:literal $($option_attrs:ident)*
            )*
        )+
    ) => {
        $crate::create_option!(@simple_option SubCommandGroup $group_name "*")
        $(
            .add_sub_option(
                $crate::create_option!(
                    $subcmd_name $subcmd_description
                    $(
                        > $option_type $option_name $option_description $($option_attrs)*
                    )*
                )
            )
        )+
    };
}

/// Create interaction command macro
///
/// Format:
///
/// ```
/// create_interaction_command!(
///     / name "Description"
///         > String subcmd "Description" [attrs]
/// )
///
/// create_interaction_command!(
///     / name
///         + subcmd subcmd_name "Description"
///             > String option_name "Description" [attrs]
//          + subcmd subcmd_name
///             > String option_name "Description" [attrs]
/// )
///
/// create_interaction_command!(
///     / name
///         - subcmdgroup subcmdgroup_name
///             + subcmd subcmd_name "Description"
///                 > String option_name "Description" [attrs]
///             + subcmd subcmd_name "Description"
///                 > String option_name "Description" [attrs]
///         - subcmdgroup subcmdgroup_name
///             + subcmd subcmd_name "Description"
///                 > String option_name "Description" [attrs]
/// )
/// ```
#[macro_export]
macro_rules! create_command {
    (
        / $name:ident
        | $description:literal
        $(
            > $option_type:ident $option_name:ident $option_description:literal $($option_attrs:ident)*
        )*
    ) => { paste::paste! {
        #[allow(unused)]
        use serenity::{
            builder::{CreateCommand, CreateCommandOption},
            model::application::CommandOptionType,
        };

        #[doc = "create the " $name " interaction command."]
        pub fn register() -> CreateCommand {
            let mut c = CreateCommand::new(stringify!($name)).description($description);

            $(
                c = c.add_option(
                    $crate::create_option!(@simple_option
                        $option_type $option_name $option_description $($option_attrs)*
                    )
                );
            )*

            return c;
        }
    } };

    (
        / $name:ident
        $(
            + $subcmd_name:ident $subcmd_description:literal
            $(
                > $option_type:ident $option_name:ident $option_description:literal $($option_attrs:ident)*
            )*
        )+
    ) => { paste::paste! {
        #[allow(unused)]
        use anyhow::{Context as _, Result};
        #[allow(unused)]
        use serenity::{
            model::application::CommandInteraction,
            builder::{CreateCommand, CreateCommandOption},
            client::Context,
            model::application::CommandOptionType,
        };

        #[doc = "Create the " $name " interaction command."]
        #[allow(unused)]
        pub fn register() -> CreateCommand {
            let mut c = CreateCommand::new(stringify!($name)).description("*");

            $(
                c = c.add_option(
                    $crate::create_option!(
                        $subcmd_name $subcmd_description
                        $(> $option_type $option_name $option_description $($option_attrs)*)*
                    )
                );
            )+

            return c;
        }

        #[doc = "Handle the " $name " command."]
        pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
            let o = command.data.options();

            let subcmd = &o.get(0).context("missing option")?;

            match subcmd.name {
                $(
                    stringify!($subcmd_name) => super::[< $name _ $subcmd_name >]::run(ctx, command).await?,
                )+
                _ => unreachable!(),
            };

            Ok(())
        }
    } };

    (
        / $name:ident
        $(
            - $subcmdgroup_name:ident
            $(
                + $subcmd_name:ident $subcmd_description:literal
                $(
                    > $option_type:ident $option_name:ident $option_description:literal $($option_attrs:ident)*
                )*
            )*
        )+
    ) => { paste::paste! {
        #[allow(unused)]
        use anyhow::{Context as _, Result};
        #[allow(unused)]
        use serenity::{
            model::application::CommandInteraction,
            builder::{CreateCommand, CreateCommandOption},
            client::Context,
            model::application::CommandOptionType,
        };

        #[doc = "Create the " $name " interaction command."]
        #[allow(unused)]
        pub fn register() -> CreateCommand {
            let mut c = CreateCommand::new(stringify!($name))
                .description("*");

            $(
                c = c.add_option(
                    $crate::create_option!(
                        $subcmdgroup_name
                        $(
                            + $subcmd_name $subcmd_description
                            $(> $option_type $option_name $option_description $($option_attrs)*)*
                        )*
                    )
                );
            )+

            return c;
        }

        #[doc = "Handle the" $name " command."]
        pub async fn run(ctx: Context, command: CommandInteraction) -> Result<()> {
            let o = command.data.options();
            let subcmdgroup = &o.get(0).context("missing option")?;

            let serenity::model::application::ResolvedValue::SubCommandGroup(subcmdgroup_value) =
                &subcmdgroup.value
            else {
                anyhow::bail!("incorrect resolved option type")
            };

            let subcmd = &subcmdgroup_value.get(0).context("missing option")?;

            match subcmdgroup.name {
                $(
                    stringify!($subcmdgroup_name) => match subcmd.name {
                        $(
                            stringify!($subcmd_name) => super::[< $name _ $subcmdgroup_name _ $subcmd_name >]::run(ctx, command).await?,
                        )*
                        _ => unreachable!(),
                    },
                )+
                _ => unreachable!(),
            };

            Ok(())
        }
    } };
}
