// @generated automatically by Diesel CLI.

diesel::table! {
    reminders (id) {
        id -> Unsigned<Bigint>,
        created_at -> Datetime,
        time -> Datetime,
        message -> Text,
        user_id -> Unsigned<Bigint>,
        channel_id -> Unsigned<Bigint>,
        guild_id -> Nullable<Unsigned<Bigint>>,
    }
}

diesel::table! {
    timezones (user_id) {
        user_id -> Unsigned<Bigint>,
        timezone -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(reminders, timezones,);
