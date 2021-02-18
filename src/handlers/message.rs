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
use crate::processing_queue::ProcessingQueue;
use crate::mini_attachment::MiniAttachment;
use crate::reactions::clock_reaction;

pub async fn handle(_: &impl EventHandler, ctx: Context, msg: Message) {
    if !msg.is_private() && msg.attachments.len() > 0 {
        let db_lock = {
            let reader = ctx.data.read().await;

            reader.get::<WRocksDb>().expect("Db instance gone").clone()
        };

        let db = db_lock.lock().await;
        let guild_id = msg.guild_id.unwrap();

        for att in &msg.attachments {
            // Store, that attachment(s) has been added to processing queue
            {
                let key_prefix = format!("guild({}):attachment({})", guild_id, att.id).into_bytes();

                db.put(&*key_prefix, b"0");

                debug!("added {} to db with status pending", att.url);
            }

            // Push mini notification to queue, so another process can take it
            {
                let queue_lock = {
                    let reader = ctx.data.read().await;

                    reader.get::<ProcessingQueue>().expect("ProcessingQueue instance gone").clone()
                };

                let mut queue = queue_lock.write().await;

                queue.push_back(MiniAttachment::from_attachment(msg.clone(), att.clone()));

                debug!("added {} to queue", att.url);
            }

            msg.react(ctx.http.clone(), clock_reaction()).await;
        }
    }
}
