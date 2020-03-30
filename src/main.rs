mod commands;
use commands::help::*;
use commands::*;

mod data;
use data::cache::{BotId, BotOwners, DefaultPrefix, GuildPrefixes, Pool, ShardManagerContainer};

mod utils;
use utils::framework::{after, dispatch_error, dynamic_prefix, log_dm};

mod db;
use db::*;

mod listeners;
use listeners::Handler;

use serenity::{framework::StandardFramework, http::Http, prelude::*};
use std::{
    collections::{HashMap, HashSet},
    env,
    sync::Arc,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("config"))
        .expect("Failed to open the config file.");

    create_db().await;

    //If a token exists in the environment, prefer to use that.
    let token;
    if let Ok(x) = env::var("DISCORD_TOKEN") {
        token = x;
    } else {
        token = settings
            .get_str("discord_token")
            .expect("discord_token not found in settings.");
    }

    let http = Http::new_with_token(&token);

    //Get the application info to use for later.
    let (owners, botid, ownerid) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id, info.owner.id)
        }
        Err(why) => panic!("Couldn't get application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.owners(owners)
                .on_mention(Some(botid))
                .dynamic_prefix(dynamic_prefix)
        })
        .on_dispatch_error(dispatch_error)
        .after(after)
        .normal_message(log_dm)
        .help(&HELP)
        .group(&SETTINGS_GROUP)
        .group(&ADMIN_GROUP)
        .group(&FUN_GROUP)
        .group(&GENERAL_GROUP);

    let mut client = Client::new_with_framework(&token, Handler, framework)
        .await
        .expect("Error creating the client.");

    //Set the cache for each channel to 100 messages.
    client
        .cache_and_http
        .cache
        .write()
        .await
        .settings_mut()
        .max_messages(100);

    //Fill the data with previously gathered and default values.
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<BotId>(botid);
        let x = vec![ownerid];
        data.insert::<BotOwners>(x);
        data.insert::<DefaultPrefix>(
            settings
                .get_str("default_prefix")
                .expect("default_prefix not found in settings."),
        );
        let map = HashMap::new();
        data.insert::<GuildPrefixes>(map);
        let pool = sqlx::SqlitePool::new("db.sqlite3")
            .await
            .expect("Failed to create DB pool");
        data.insert::<Pool>(pool);
    }

    client
        .start_autosharded()
        .await
        .expect("Failed to start the client.")
}
