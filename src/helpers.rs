use serenity::model::channel::Attachment;
use serenity::model::id::{AttachmentId, GuildId};

use crate::queue_entry::QueueEntry;

#[inline]
fn attachment_key(att_id: &AttachmentId) -> &str {
    format!("attch:{:x}", att_id.as_u64()).as_ref()
}

trait RocksDbKey {
    fn to_db_key(&self) -> &str;
}

impl RocksDbKey for GuildId {
    fn to_db_key(&self) -> &str {
        format!("guild({:x})", self.as_u64()).as_ref()
    }
}

impl RocksDbKey for QueueEntry {
    fn to_db_key(&self) -> &str {
        attachment_key(&self.att_id)
    }
}

impl RocksDbKey for Attachment {
    fn to_db_key(&self) -> &str {
        attachment_key(&self.id)
    }
}

impl RocksDbKey for AttachmentId {
    fn to_db_key(&self) -> &str {
        attachment_key(self)
    }
}

pub(crate) enum AttachmentStatus {
    Pending = 0,
    Processing = 1,
    Processed = 2,
}
