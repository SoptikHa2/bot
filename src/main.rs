mod commands;
use commands::*;

mod data;
use data::*;

mod util;
use util::*;

mod db;
use db::*;

mod events;
use events::Handler;

use dotenv::dotenv;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use serenity::{framework::StandardFramework, model::prelude::*, prelude::*};

fn main() {
    dotenv().ok();

    create_db();

    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment");
    let mut client = Client::new(&token, Handler).expect("Error creating client");

    let id: Option<UserId>;

    let owners = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            let mut data = client.data.write();
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
            data.insert::<BotId>(info.id);
            id = Some(info.id);
            let x = vec![info.owner.id];
            data.insert::<BotOwners>(x);

            set
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    client
        .cache_and_http
        .cache
        .write()
        .settings_mut()
        .max_messages(100);

    client.with_framework(
        StandardFramework::new()
            .configure(|c| {
                c.owners(owners).on_mention(id).dynamic_prefix(|_, msg| {
                    if msg.is_private() {
                        return Some("!".to_string());
                    }
                    if let Some(guild_id) = msg.guild_id {
                        let prefix =
                            get_prefix(&guild_id).map_or_else(|_| "!".to_string(), |pref| pref);
                        println!("{:?}", prefix);
                        return Some(prefix);
                    } else {
                        return Some("!".to_string());
                    }
                })
            })
            .normal_message(log_dm)
            .group(&CONFIG_GROUP)
            .group(&ADMIN_GROUP)
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
