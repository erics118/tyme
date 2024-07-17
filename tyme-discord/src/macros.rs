//! Macros.

// #[macro_export]
// macro_rules! commands {
//     // Match when there are no subcommands
//     (@module $cmd:ident) => {
//         pub mod $cmd;
//     };

//     // Match when there are subcommands
//     (@module $cmd:ident[$($subcmd:ident),* $(,)?]) => { paste::paste! {
//         $(
//             pub mod [< $cmd _ $subcmd  >];
//         )*

//         pub mod $cmd {
//             #[poise::command(
//                 slash_command,
//                 subcommands(
//                     "crate::commands::reminders_delete::reminders_delete"
// $subcmd                 )
//             )]
//             pub async fn $cmd(_: $crate::Context<'_>) -> anyhow::Result<(),
// $crate::Error> {                 Ok(())
//             }
//         }
//     } };

//     (@all $cmd:ident) => {
//         $cmd:$cmd();
//     };

//     (
//         @all
//         $cmd:ident[
//             $($subcmd:ident),+
//         ]
//     ) => {
//     };

//     (
//         $($cmd:ident
//             $(
//                 [
//                     $($subcmd:ident),+
//                     $(,)?
//                 ]
//             )?
//         ),+
//         $(,)?
//     ) => {
//         $(
//             commands!(@module $cmd$( [$($subcmd),* ] )?);
//         )*

//         /// Utility function to get all commands.
//         pub fn all() -> Vec<poise::Command<Data, Error>> {
//             // vec![
//             //     $(
//             //         commands!(@all $cmd$( [$($subcmd),*] )?)
//             //     )+
//             // ]
//             vec![]
//         }
//     };
// }

#[macro_export]
macro_rules! commands {
    (
        $($cmd:ident
            $(
                [
                    $($subcmd:ident),+
                    $(,)?
                ]
            )?
        ),+
        $(,)?
    ) => { paste::paste!{
        $(
            pub mod $cmd;

            $(
                $(
                    pub mod [< $cmd _ $subcmd >];
                )+
            )?
        )+

        /// Utility function to get all commands.
        pub fn all() -> Vec<poise::Command<$crate::types::Data, $crate::types::Error>> {
            vec![
                $(
                    $cmd::$cmd(),
                )+
            ]
        }
    } };
}
