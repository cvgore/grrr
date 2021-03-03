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
use serenity::prelude::EventHandler;
use serenity::static_assertions::_core::time::Duration;
use tracing::{debug, error, info};
use tracing::field::debug;
use tracing_subscriber::{
    EnvFilter,
    FmtSubscriber,
};

use crate::db::WRocksDb;
use crate::helpers::AttachmentStatus;
use crate::processing_queue::ProcessingQueue;
use crate::queue_entry::QueueEntry;
use crate::reactions::clock_reaction;
use crate::UploadNotif;

pub async fn handle(_: &impl EventHandler, ctx: Context, msg: Message) {
    if !msg.is_private() && !msg.attachments.is_empty() {
        let (db_lock, upload_notif) = {
            let reader = ctx.data.read().await;

            (
                reader.get::<WRocksDb>().expect("Db instance gone").clone(),
                reader.get::<UploadNotif>().expect("upload notif gone").clone(),
            )
        };

        let db = db_lock.lock().await;
        let guild_id = msg.guild_id.unwrap();

        for att in &msg.attachments {
            let queue_lock = {
                let reader = ctx.data.read().await;

                reader.get::<ProcessingQueue>().expect("ProcessingQueue instance gone").clone()
            };

            let mut queue = queue_lock.write().await;

            let guild_chan = {
                let ch = msg.channel(ctx.cache.clone()).await;

                if let Some(ch) = ch {
                    ch.guild()
                } else {
                    None
                }
            };

            if guild_chan.is_some() {
                debug!("got guild 2")
            } else {
                debug!("guild missing");
            }

            queue.push_back(QueueEntry::from_gateway(&msg, att.clone(), guild_chan));

            debug!("added {} to queue", att.url);

            // Store, that attachment(s) has been added to processing queue
            {
                db.set_attch_status(&guild_id, &att.id, AttachmentStatus::Pending);
            }
        }

        upload_notif.notify_one();
    }
}
