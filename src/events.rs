use serenity::{
    async_trait,
    client::{bridge::gateway::event::ShardStageUpdateEvent, Context, EventHandler},
    framework::standard::{macros::hook, CommandResult, DispatchError, Reason},
    model::{
        channel::Message,
        prelude::{Activity, OnlineStatus, Ready},
    },
};
use tracing::{info, warn};

use crate::constants;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _: Context, _: Message) {}

    async fn ready(&self, ctx: Context, _: Ready) {
        info!("Nyan v{} is ready", &constants::CONFIG.bot.version);

        ctx.set_presence(
            Some(Activity::playing("nyaaaan~")),
            OnlineStatus::DoNotDisturb,
        )
        .await
    }

    async fn shard_stage_update(&self, _: Context, shard: ShardStageUpdateEvent) {
        info!(
            "Shard {} has gone from {} to {}",
            shard.shard_id, shard.old, shard.new
        );
    }
}

#[hook]
pub async fn on_prefix_only(ctx: &Context, msg: &Message) {
    let _ = msg
        .channel_id
        .say(
            &ctx.http,
            format!(
                "`Hellow, {0} is my prefix and you can get a list of commands by doing {0}help.`",
                msg.content
            ),
        )
        .await;
}

#[hook]
pub async fn before(_: &Context, _: &Message, _: &str) -> bool {
    true // if `before` returns false, command processing doesn't happen.
}

#[hook]
pub async fn after(_: &Context, _: &Message, _: &str, _: CommandResult) {}

#[hook]
pub async fn unrecognised_command(_: &Context, _: &Message, _: &str) {}

#[hook]
pub async fn normal_message(_: &Context, _: &Message) {}

#[hook]
pub async fn on_dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::OnlyForOwners => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "`This command is for owners.`")
                .await;
        }
        DispatchError::OnlyForDM => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "`This command is for DMs.`")
                .await;
        }
        DispatchError::OnlyForGuilds => {
            let _ = msg
                .channel_id
                .say(&ctx.http, "`This command is for servers.`")
                .await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!("`I need {} arguments but was supplied {}.`", min, given),
                )
                .await;
        }
        DispatchError::CheckFailed(_, reason) => {
            if let Reason::User(e) = reason {
                let _ = msg.channel_id.say(&ctx.http, format!("`{}`", e)).await;
            }
        }
        _ => {
            warn!("Unhandled dispatch error: {:?}", error);
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    "`Unknown error has occurred, this has been reported to the developers.`",
                )
                .await;
        }
    }
}
