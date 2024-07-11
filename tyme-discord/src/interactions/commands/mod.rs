//! Interaction commands.

use crate::interaction_commands;

interaction_commands! {
    test,
    remind,
    reminders[list, delete],
    timezone[get, set, delete],
}

/// hi
pub mod test_subcmdgroup1_subcmd11;

/// hi
pub mod test_subcmdgroup1_subcmd12;

/// hi
pub mod test_subcmdgroup2_subcmd21;
