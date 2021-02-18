use serenity::model::channel::ReactionType;

pub fn magnet_reaction() -> ReactionType {
    ReactionType::Unicode("🧲".to_string())
}

pub fn clock_reaction() -> ReactionType {
    ReactionType::Unicode("⏲️".to_string())
}
