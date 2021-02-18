use serenity::model::channel::{Message, Attachment};

pub struct MiniAttachment {
    pub url: String,
    pub msg: Message,
    pub filename: String,
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
