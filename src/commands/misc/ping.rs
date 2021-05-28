use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

use std::time::Instant;
#[command]
#[description("Check websocket latency")]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let now = Instant::now();
    msg.channel_id.broadcast_typing(&ctx.http).await?;
    let end = now.elapsed().as_millis();

    msg.channel_id
        .say(&ctx.http, format!("`{}ms websocket latency.`", end))
        .await?;

    Ok(())
}
