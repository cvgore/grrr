use std::fs;
use std::sync::Arc;

use reqwest::Client;
use serde_json::json;
use serenity::http::Http;
use serenity::prelude::TypeMap;
use tokio::process::Command;
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, error, info};

use crate::db::WRocksDb;
use crate::processing_queue::ProcessingQueue;
use crate::queue_entry::QueueEntry;
use crate::reactions::{clock_reaction, magnet_reaction};
use std::collections::HashMap;

pub async fn process_queue(data_lock: Arc<RwLock<TypeMap>>, discord_http: Arc<Http>, rclone_http: Arc<Client>) {
    let queue_lock = {
        let mut data = data_lock.read().await;

        data.get::<ProcessingQueue>().unwrap().clone()
    };

    while let Some(entry) = {
        let mut queue = queue_lock.write().await;

        queue.pop_back()
    } {
        if false {
            discord_http.delete_reaction(
                entry.channel_id.0,
                entry.msg_id.0,
                None,
                &clock_reaction(),
            ).await;
        }

        debug!("got attachment {}", &entry.filename);

        {
            let data = data_lock.read().await;
            let db_lock = data.get::<WRocksDb>().unwrap();

            process_file(&entry, db_lock.clone(), rclone_http.clone()).await;
        }

        discord_http.create_reaction(
            entry.channel_id.0,
            entry.msg_id.0,
            &magnet_reaction(),
        ).await;

        debug!("file written {}", &entry.filename)
    }
}

pub async fn process_file(entry: &QueueEntry, db_lock: Arc<Mutex<rocksdb::DB>>, rclone_http: Arc<Client>) {
    let db = db_lock.lock().await;

    let flake = entry.url.replace("https://cdn.discordapp.com/attachments/", "");

    let mut document = HashMap::new();
    let drive_file_name = format!("{}_{}", entry.att_id, entry.filename);

    debug!("discord flake: {}", flake);

    document.insert("srcFs", "discord:");
    document.insert("srcRemote", &flake);
    document.insert("dstFs", "gdrive:");
    document.insert("dstRemote", &drive_file_name);

    rclone_http.post("http://localhost:5572/operations/copyfile")
        .json(&document)
        .send()
        .await.unwrap();
}
