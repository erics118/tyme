/// Get the value from an option.
#[macro_export]
macro_rules! get_option_value {
    ($options:ident $index:tt $option_type:ident ) => { {
        #[allow(unused)]
        use anyhow::Context as _;

        let serenity::model::application::ResolvedValue::$option_type(value) = &$options.get($index).context("missing option")?.value else {
            anyhow::bail!("incorrect resolved option type")
        };

        value
    } };

    // subcommands for autocomplete
    ($options:ident $index:tt . [Autocomplete] ) => { {
        #[allow(unused)]
        use anyhow::Context as _;

        let value = $crate::get_option_value!($options $index SubCommand);

        let serenity::model::application::ResolvedValue::Autocomplete{value, kind} = &value.get($index).context("missing option")?.value else {
            anyhow::bail!("incorrect resolved option type")
        };

        (value.clone(), kind.clone())
    } };

    // subcommands
    ($options:ident $index:tt . [$($nested_type:ident)+] ) => { {
        let value = $crate::get_option_value!($options $index SubCommand);

        let mut index = 0;

        ( $( {
            let value = $crate::get_option_value!(value index $nested_type);

            // value is used in the next repetition
            #[allow(unused_assignments)]
            index += 1;
            value.clone()
        }, )+ )
    } };

    // subcommand groups
    ($options:ident $index:tt . [[$($nested_type:ident)+]] ) => { {
        let value = $crate::get_option_value!($options $index SubCommandGroup);

        let value = $crate::get_option_value!(value $index SubCommand);

        let mut index = 0;

        ( $( {
            let value = $crate::get_option_value!(value index $nested_type);

            // value is used in the next repetition
            #[allow(unused_assignments)]
            index += 1;
            value.clone()
        }, )+ )
    } };
}

/// Get an option from a command.
#[macro_export]
macro_rules! get_options {
    (
        $command:ident,
        $(
            // Only one of these two can be matched, because get_option_value
            // will fail if both exist.
            $( $option_type:ident )?
            $( . [$($nested_type:ident),+])?
            $( . [[$($nested_type2:ident),+]])?
        ),+
        $(,)?
    ) => { {
        let options = $command.data.options();
        let mut index = 0;

        ( $( {
            let value = $crate::get_option_value!(
                options
                index
                $( $option_type )?
                $( . [$($nested_type)+])?
                $( . [[$($nested_type2)+]])?
            );

            // value is used in the next repetition
            #[allow(unused_assignments)]
            index += 1;
            value.clone()
        }, )+ )
    } };
}
