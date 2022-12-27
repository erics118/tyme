use serenity::{model::id::UserId, prelude::TypeMapKey};

pub struct SelfId;

impl TypeMapKey for SelfId {
    type Value = UserId;
}
