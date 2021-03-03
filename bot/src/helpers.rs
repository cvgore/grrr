use serenity::model::channel::Attachment;
use serenity::model::id::{AttachmentId, GuildId};

use crate::queue_entry::QueueEntry;
use serenity::prelude::Context;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::db::{BotStorage, WRocksDb};
use serenity::async_trait;

#[inline]
fn attachment_key(att_id: &AttachmentId) -> String {
    format!("attch:{:x}", att_id.as_u64())
}

pub trait RocksDbKey {
    fn to_db_key(&self) -> String;
}

#[async_trait]
pub trait ContextHelpers {
    async fn get_db(&self) -> Arc<Mutex<BotStorage>>;
}

#[async_trait]
impl ContextHelpers for Context {
    async fn get_db(&self) -> Arc<Mutex<BotStorage>> {
        let data_lock = self.data.read().await;

        data_lock.get::<WRocksDb>().expect("db instance gone").clone()
    }
}

impl RocksDbKey for GuildId {
    fn to_db_key(&self) -> String {
        format!("guild({:x})", self.as_u64())
    }
}

impl RocksDbKey for QueueEntry {
    fn to_db_key(&self) -> String {
        attachment_key(&self.att_id)
    }
}

impl RocksDbKey for Attachment {
    fn to_db_key(&self) -> String {
        attachment_key(&self.id)
    }
}

impl RocksDbKey for AttachmentId {
    fn to_db_key(&self) -> String {
        attachment_key(self)
    }
}

pub enum AttachmentStatus {
    Pending = 0,
    Processing = 1,
    Processed = 2,
}

impl AsRef<[u8]> for AttachmentStatus {
    fn as_ref(&self) -> &[u8] {
        match self {
            AttachmentStatus::Pending => &[0u8],
            AttachmentStatus::Processing => &[1u8],
            AttachmentStatus::Processed => &[2u8]
        }
    }
}
