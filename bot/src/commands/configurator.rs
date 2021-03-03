use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{CommandResult, Args};
use serenity::framework::standard::macros::command;
use tracing::info;
use serenity::utils::MessageBuilder;
use crate::helpers::ContextHelpers;
use serenity::http::routing::RouteInfo::CreateMessage;

#[command]
pub async fn configure(ctx: &Context, msg: &Message, mut _args: Args) -> CommandResult<()> {
    if msg.author.bot {
        info!("contacted by bot using configure: {}", msg.author.name);

        return Ok(());
    }

    let db = {
        let db_lock = ctx.get_db().await;

        db_lock.lock().await
    };

    let msg = MessageBuilder::new()
        .push("Please visit")
        .push_line(format!("{base_url}/dashboard/{guild_id}?s={signature}"))

    msg.reply(&ctx.http, ).await;

    Ok(())
}

fn
