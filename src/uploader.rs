use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use regex::Regex;
use reqwest::Client;
use serde_json::json;
use serenity::http::Http;
use serenity::prelude::TypeMap;
use tokio::process::Command;
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info};

use crate::db::{WRocksDb, BotStorage};
use crate::processing_queue::ProcessingQueue;
use crate::queue_entry::QueueEntry;
use crate::reactions::{clock_reaction, magnet_reaction};
use crate::helpers::AttachmentStatus;
use crate::db;

pub async fn process_queue(data_lock: Arc<RwLock<TypeMap>>, discord_http: Arc<Http>, rclone_http: Arc<Client>) {
    while let Some(entry) = {
        let queue_lock = {
            let data = data_lock.read().await;

            data.get::<ProcessingQueue>().unwrap().clone()
        };

        let mut queue = queue_lock.write().await;

        queue.pop_back()
    } {
        debug!("got attachment {}", &entry.fname);

        {
            let data = data_lock.read().await;
            let db_lock = data.get::<WRocksDb>().unwrap();

            process_file(&entry, db_lock.clone(), rclone_http.clone()).await;
        }

        discord_http.create_reaction(
            entry.ch_id.0,
            entry.msg_id.0,
            &magnet_reaction(),
        ).await;

        debug!("file written {}", &entry.fname)
    }
}

pub async fn process_file(entry: &QueueEntry, db_lock: Arc<Mutex<BotStorage>>, rclone_http: Arc<Client>) {
    let db = db_lock.lock().await;

    db.set_attch_status(&entry.guild_id.unwrap(), &entry.att_id, AttachmentStatus::Processing);

    let flake = entry.url.replace("https://cdn.discordapp.com/attachments/", "");

    let drive_file_name = get_drive_file_path(&entry).await;

    debug!("discord flake: {}", flake);
    debug!("drive file name: {}", drive_file_name);
    debug!("discord channel name: {}", entry.ch_name.clone().unwrap_or_else(|| "#NULL".to_string()));

    let mut document = HashMap::new();
    document.insert("srcFs", "discord:");
    document.insert("srcRemote", &flake);
    document.insert("dstFs", "gdrive:");
    document.insert("dstRemote", &drive_file_name);

    rclone_http.post("http://localhost:5572/operations/copyfile")
        .json(&document)
        .send()
        .await
        .unwrap();

    db.set_attch_status(&entry.guild_id.unwrap(), &entry.att_id, AttachmentStatus::Processed);
}

async fn get_drive_file_path(entry: &QueueEntry) -> String {
    let mut path = String::new();

    if let Some(mut ch_name) = entry.ch_name.clone() {
        ch_name.retain(only_utf8_simple);

        debug!("filtered chan name: {}", &ch_name);

        path.push_str(&ch_name);
        path.push('/');
    } else {
        path.push_str("unknown_channel/")
    }

    let mut fname = entry.fname.clone();
    fname.retain(only_utf8_simple);
    let fname = format!("{}_{}", &entry.att_id, &fname);

    path.push_str(&fname);

    path
}

#[inline]
fn only_utf8_simple(c: char) -> bool {
    char::is_alphabetic(c) || char::is_digit(c, 10) || ", ()-[]{};?".contains(c)
}
