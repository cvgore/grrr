use serenity::prelude::TypeMapKey;
use crate::queue_entry::QueueEntry;
use std::collections::VecDeque;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct ProcessingQueue;

impl TypeMapKey for ProcessingQueue {
    type Value = Arc<RwLock<VecDeque<QueueEntry>>>;
}
