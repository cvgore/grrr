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

use crate::db::WRocksDb;
use crate::lazy_static;
use crate::processing_queue::ProcessingQueue;
use crate::queue_entry::QueueEntry;
use crate::reactions::{clock_reaction, magnet_reaction};

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
                entry.ch_id.0,
                entry.msg_id.0,
                None,
                &clock_reaction(),
            ).await;
        }

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

pub async fn process_file(entry: &QueueEntry, db_lock: Arc<Mutex<rocksdb::DB>>, rclone_http: Arc<Client>) {
    let db = db_lock.lock().await;

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
        .await.unwrap();
}

async fn get_drive_file_path(entry: &QueueEntry) -> String {
    lazy_static! {
        static ref FNAME_REGEXP: Regex = Regex::new(r#"[\w, ()-]"#).unwrap();
    }

    let mut path = String::new();

    if let Some(mut ch_name) = entry.ch_name.clone() {
        ch_name.retain(|c| FNAME_REGEXP.is_match(&c.to_string()));

        debug!("filtered chan name: {}", ch_name);

        path.push_str(&ch_name);
        path.push('/');
    }

    let fname = format!("{}_{}", entry.att_id, entry.fname);

    path.push_str(&fname);

    path
}
