use serenity::model::channel::ReactionType;

#[inline]
pub fn magnet_reaction() -> ReactionType {
    ReactionType::Unicode("🧲".to_string())
}

#[inline]
pub fn clock_reaction() -> ReactionType {
    ReactionType::Unicode("⏲️".to_string())
}
