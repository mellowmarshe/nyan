use std::{error::Error, time::Duration};

use serenity::{
    client::Context,
    model::channel::{Message, ReactionType},
};

pub async fn wait_for_reaction(
    ctx: &Context,
    emote: ReactionType,
    msg: &Message,
    message: String,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let confirmation = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content(format!("```{}```", message))
                .reactions(Some(emote.clone()))
        })
        .await?;

    let collector = confirmation
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(30))
        .author_id(msg.author.id)
        .filter(move |r| r.emoji.as_data() == emote.as_data())
        .await;

    confirmation.delete(&ctx.http).await?;

    Ok(!collector.is_none())
}
