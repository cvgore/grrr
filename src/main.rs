use std::{collections::HashSet, env, sync::Arc, thread};
use std::collections::VecDeque;
use std::fs;

use reqwest;
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
use serenity::model::id::{GuildId, MessageId};
use serenity::static_assertions::_core::time::Duration;
use tracing::{debug, error, info};
use tracing::field::debug;
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
};

mod handlers;
// mod commands;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct WRocksDb;

impl TypeMapKey for WRocksDb {
    type Value = Arc<Mutex<rocksdb::DB>>;
}

struct MiniAttachment {
    url: String,
    msg: Message,
    filename: String,
}

impl MiniAttachment {
    pub fn from_attachment(msg: Message, att: Attachment) -> Self {
        MiniAttachment {
            msg,
            url: att.url,
            filename: att.filename,
        }
    }
}

struct ProcessingQueue;

impl TypeMapKey for ProcessingQueue {
    type Value = Arc<RwLock<VecDeque<MiniAttachment>>>;
}

struct Handler;

enum AttachmentStatus {
    Pending = 0,
    Processing = 1,
    Processed = 2,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        handlers::message::handle(self, ctx, msg);
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}, accessing {} guild(s)", ready.user.name, ready.guilds.len());
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

// #[group]
// #[commands(multiply, ping, quit)]
// struct General;

const MAGNET_REACTION: ReactionType = ReactionType::Unicode("🧲".to_string());
const CLOCK_REACTION: ReactionType = ReactionType::Unicode("⏲️".to_string());

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environ");

    let shard_count = {
        let count = env::var("SHARD_COUNT")
            .expect("Expected a shard count in the environ");

        count.parse::<u64>().expect("Expected shard count as a integer")
    };

    let http = Http::new_with_token(&token);

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
            .allow_dm(false)
            .ignore_bots(true)
            .ignore_webhooks(true)
            .prefix(";"));
    // .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .framework(framework)
        .intents(
            GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
        )
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let path = "grrr.db";

    {
        let db = rocksdb::DB::open_default(path)
            .expect(&format!("Couldn't open db file: {}", path));

        let mut data = client.data.write().await;
        data.insert::<WRocksDb>(Arc::new(Mutex::new(db)));
    }

    {
        let mut data = client.data.write().await;
        data.insert::<ProcessingQueue>(Arc::new(RwLock::new(VecDeque::new())));
    }

    let new_data = client.data.clone();
    let cache_http = client.cache_and_http.http.clone();

    let handle = tokio::task::spawn(async move {
        let mut data = new_data.read().await;

        let queue_lock = {
            data.get::<ProcessingQueue>().unwrap().clone()
        };

        loop {
            let mut queue = queue_lock.write().await;

            while !queue.is_empty() {
                let att = queue.pop_back().unwrap();
                att.msg.delete_reaction_emoji(cache_http.clone(), CLOCK_REACTION).await;

                debug!("got attachment {}", &att.filename);

                let blob = reqwest::get(&att.url)
                    .await.unwrap()
                    .bytes()
                    .await.unwrap();

                fs::write(format!("/tmp/{}", &att.filename), blob).unwrap();

                att.msg.react(cache_http.clone(), MAGNET_REACTION).await;

                debug!("file written {}", &att.filename)
            }

            tokio::time::sleep(Duration::from_millis(1000)).await;
        };
    });

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ^c handler");
        shard_manager.lock().await.shutdown_all().await;
        handle.abort();
    });

    if let Err(why) = client.start_shards(shard_count).await {
        error!("Client error: {:?}", why);
    }
}
