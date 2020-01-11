use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use serenity::{client::bridge::gateway::ShardManager, model::prelude::*, prelude::*};
use std::sync::Arc;

pub struct DatabaseConnection;

impl TypeMapKey for DatabaseConnection {
    type Value = Pool<ConnectionManager<SqliteConnection>>;
}

pub struct BotOwners;

impl TypeMapKey for BotOwners {
    type Value = Vec<UserId>;
}

pub struct BotId;

impl TypeMapKey for BotId {
    type Value = UserId;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub enum LogType {
	MessageDeleted = 1 << 1,
	MessageEdited = 1 << 2,
	UserJoined = 1 << 3,
	UserLeft = 1 << 4,
	All = (1 << 4) - 1,
}