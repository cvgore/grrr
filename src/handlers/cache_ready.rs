use std::{collections::HashSet, env, sync::Arc, thread};
use std::collections::VecDeque;
use std::fs;

use reqwest;
use rocksdb::Options;
use serenity::{
    async_trait,
    client::bridge::gateway::ShardManager,
    framework::{
        standard::macros::group,
        StandardFramework,
    },
    http::Http,
    model::{event::ResumedEvent, gateway::Ready},
    prelude::*,
};
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::model::channel::{Attachment, Message, Reaction, ReactionType};
use serenity::model::guild::Guild;
use serenity::model::id::{GuildId, MessageId};
use serenity::prelude::EventHandler;
use serenity::static_assertions::_core::time::Duration;
use tracing::{debug, error, info};
use tracing::field::debug;
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
};

use crate::db::WRocksDb;
use crate::processing_queue::ProcessingQueue;
use crate::queue_entry::QueueEntry;
use crate::reactions::clock_reaction;
use crate::UploadNotif;

pub async fn handle(_: &impl EventHandler, ctx: Context, guilds: Vec<GuildId>) {
    info!("cache ready, {} guilds", guilds.len());

    let db_lock = {
        let reader = ctx.data.read().await;

        reader.get::<WRocksDb>().expect("Db instance gone").clone()
    };

    let mut db = db_lock.lock().await;

    for guild in guilds {
        let cf_name = guild.to_db_key();
        // Missing/new/not loaded previously
        if db.cf_handle(cf_name).is_none() {
            match db.create_cf(&cf_name, &Options::default()) {
                Err(why) => error!("failed to create CF '{}' - reason: {:?}", cf_name, why),
                Ok(_) => debug!("created CF '{}'", cf_name)
            };
        } else {
            debug!("skipped creating live CF '{}'", cf_name);
        }
    }
}
