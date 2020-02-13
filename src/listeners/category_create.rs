use crate::data::db::LogType;
use crate::db::*;
use log::error;
use serenity::{model::prelude::*, prelude::*};
use std::sync::Arc;

pub fn category_create(ctx: Context, category: Arc<RwLock<ChannelCategory>>) {
    let c = category.read();
    let guildid =
        c.id.to_channel(&ctx)
            .unwrap()
            .guild()
            .unwrap()
            .read()
            .guild_id;

    let log_channel = match get_log_channel(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    let log_type = match get_log_type(&guildid) {
        Ok(l) => l,
        Err(_) => {
            return;
        }
    };

    if log_type & LogType::CategoryCreated as i64 != LogType::CategoryDeleted as i64 {
        return;
    }

    if let Err(e) = log_channel.say(&ctx.http, format!("Category created: {}", c.name)) {
        error!("{:?}", e);
    }
}
