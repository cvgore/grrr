use std::sync::Arc;

use rocksdb::Options;
use serenity::model::channel::Attachment;
use serenity::model::id::{GuildId, AttachmentId};
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
use tracing::field::debug;

use crate::helpers::AttachmentStatus;
use tracing::{debug, error};
use crate::helpers::RocksDbKey;

pub struct WRocksDb;

impl TypeMapKey for WRocksDb {
    type Value = Arc<Mutex<BotStorage>>;
}

pub struct BotStorage {
    path: String,
    db: rocksdb::DB,
}

impl BotStorage {
    pub fn new() -> Self {
        let path = "grrr.db";

        let cf_names = rocksdb::DB::list_cf(&Options::default(), &path)
            .unwrap_or_else(|_| panic!("Couldn't list CF file: {}", &path));

        let db = rocksdb::DB::open_cf(&Options::default(), &path, cf_names)
            .unwrap_or_else(|_| panic!("Couldn't open db file: {}", &path));

        BotStorage {
            path: path.to_string(),
            db,
        }
    }

    pub fn create_guild_ns_missing(&mut self, guilds: Vec<GuildId>) {
        for guild in &guilds {
            let cf_name = guild.to_db_key();

            // Missing/new/not loaded previously
            if self.db.cf_handle(&cf_name).is_none() {
                match self.db.create_cf(&cf_name, &Options::default()) {
                    Err(why) => error!("failed to create guild NS '{}' - reason: {:?}", &cf_name, why),
                    Ok(_) => debug!("created guild NS '{}'", &cf_name)
                };
            } else {
                debug!("skipped creating live guild NS '{}'", &cf_name);
            }
        }
    }

    pub fn set_attch_status(&self, guild_id: &GuildId, att: &AttachmentId, status: AttachmentStatus) {
        self.db.put_cf(
            self.db.cf_handle(&guild_id.to_db_key()).unwrap(),
            att.to_db_key(),
            status,
        );
    }
}
