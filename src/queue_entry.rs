use serenity::model::channel::{Attachment, Channel, GuildChannel, Message};
use serenity::model::guild::Guild;
use serenity::model::id::{AttachmentId, ChannelId, GuildId, MessageId};

pub struct QueueEntry {
    pub url: String,
    pub fname: String,
    pub msg_id: MessageId,
    pub ch_id: ChannelId,
    pub guild_id: Option<GuildId>,
    pub att_id: AttachmentId,
    pub ch_name: Option<String>,
}

impl QueueEntry {
    pub fn from_gateway(msg: &Message, att: Attachment, chan: Option<GuildChannel>) -> Self {
        let mut ch_name = {
            match chan {
                Some(chan) => Some(chan.name),
                _ => None,
            }
        };

        QueueEntry {
            att_id: att.id,
            msg_id: msg.id,
            ch_id: msg.channel_id,
            guild_id: msg.guild_id,
            url: att.url,
            fname: att.filename,
            ch_name,
        }
    }
}
