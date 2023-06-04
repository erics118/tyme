-- Add migration script here
CREATE TABLE IF NOT EXISTS reminders (
    id          bigint unsigned PRIMARY KEY NOT NULL AUTO_INCREMENT,
    created_at  datetime NOT NULL,
    time        datetime NOT NULL,
    message     text NOT NULL,
    user_id     bigint unsigned NOT NULL,
    channel_id  bigint unsigned NOT NULL,
    guild_id    bigint unsigned
);

CREATE TABLE IF NOT EXISTS timezones (
    user_id bigint unsigned PRIMARY KEY NOT NULL,
    timezone text NOT NULL
);
