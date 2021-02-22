use serenity::model::channel::Attachment;
use serenity::model::id::GuildId;

use crate::queue_entry::QueueEntry;

trait RocksDbKey {
    fn to_db_key(&self) -> &str;
}

impl RocksDbKey for GuildId {
    pub fn to_db_key(&self) -> &str {
        format!("guild({:x})", self.0).as_ref()
    }
}

impl RocksDbKey for QueueEntry {
    pub fn to_db_key(&self) -> &str {
        format!("attch:{:x}", self.att_id.as_u64()).as_ref()
    }
}

impl RocksDbKey for Attachment {
    pub(crate) fn to_db_key(&self) -> &str {
        format!("attch:{:x}", self.att_id.as_u64()).as_ref()
    }
}

pub(crate) enum AttachmentStatus {
    Pending = 0,
    Processing = 1,
    Processed = 2,
}
