use crate::create_command;

create_command! {
    / test
        - subcmdgroup1
            + subcmd11 "subcmd 11 desc"
                > String val "Phrase" required
            + subcmd12 "subcmd 12 desc"
        - subcmdgroup2
            + subcmd21 "subcmd 21 desc"
                > String val "Ugh" required
}
