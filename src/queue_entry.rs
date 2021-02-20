use serenity::model::channel::{Attachment, Message};
use serenity::model::id::{ChannelId, GuildId, MessageId, AttachmentId};

pub struct QueueEntry {
    pub url: String,
    pub filename: String,
    pub msg_id: MessageId,
    pub channel_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub att_id: AttachmentId,
}

impl QueueEntry {
    pub fn from_gateway(msg: &Message, att: Attachment) -> Self {
        QueueEntry {
            att_id: att.id,
            msg_id: msg.id,
            channel_id: msg.channel_id,
            guild_id: msg.guild_id,
            url: att.url,
            filename: att.filename,
        }
    }
}
