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
use serenity::static_assertions::_core::time::Duration;
use tokio::sync::Notify;
use tracing::{debug, error, info};
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
};

use db::WRocksDb;
use processing_queue::ProcessingQueue;

use crate::db::BotStorage;
use crate::queue_entry::QueueEntry;
use crate::reactions::{clock_reaction, magnet_reaction};
use serenity::model::gateway::Activity;

mod handlers;
mod db;
mod processing_queue;
mod queue_entry;
mod reactions;
mod uploader;
mod helpers;
mod upload;
mod ver;
mod commands;
mod configurator;

use commands::configurator::*;

// mod commands;

pub struct ShardManagerContainer;

struct UploadNotif;

impl TypeMapKey for UploadNotif {
    type Value = Arc<Notify>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, guilds: Vec<GuildId>) {
        handlers::cache_ready::handle(self, ctx, guilds).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        handlers::guild_create::handle(self, ctx, guild, is_new).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        handlers::message::handle(self, ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening("for files to backup")).await;

        info!("Connected as {}, accessing {} guild(s)", ready.user.name, ready.guilds.len());
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group("general")]
#[commands(configure)]
struct General;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let discord_token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environ");

    let rclone_url = env::var("RCLONE_URL")
        .expect("Expected rclone API url in the environ");

    {
        let rclone_http = upload::client::Client::new(rclone_url);

        if let Err(why) = rclone_http.ping().await {
            panic!("couldnt contact rclone api: {:?}", why);
        }
    }

    let http = Http::new_with_token(&discord_token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c
            .owners(owners)
            .prefix("&"))
    .group(&GENERAL_GROUP);

    let mut client = Client::builder(&discord_token)
        .framework(framework)
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::GUILDS
        )
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    {
        let mut db = BotStorage::new();

        let mut data = client.data.write().await;
        data.insert::<WRocksDb>(Arc::new(Mutex::new(db)));
    }

    {
        let mut data = client.data.write().await;
        data.insert::<ProcessingQueue>(Arc::new(RwLock::new(VecDeque::new())));
    }

    let upload_notif = Arc::new(Notify::new());

    {
        let mut data = client.data.write().await;
        data.insert::<UploadNotif>(upload_notif.clone());
    }

    let h_uploader = {
        let data = client.data.clone();
        let discord_http = client.cache_and_http.http.clone();
        let rclone_http = Arc::new(reqwest::Client::new());

        tokio::spawn(async move {
            loop {
                upload_notif.notified().await;

                uploader::process_queue(
                    data.clone(),
                    discord_http.clone(),
                    rclone_http.clone(),
                ).await;
            }
        })
    };

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ^c handler");
        shard_manager.lock().await.shutdown_all().await;
        h_uploader.abort();
    });

    if let Err(why) = client.start_autosharded().await {
        error!("Client error: {:?}", why);
    }
}
