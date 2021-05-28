use rand::Rng;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::{Message, ReactionType},
    prelude::*,
};

use crate::{
    constants,
    database::ConnectionPool,
    models::cat::{Cat, Color},
    utils::{checks::*, collectors, images},
};

#[command]
#[checks(DoesntHaveCat)]
#[min_args(2)]
#[description("Create your character")]
#[usage("[grey|white|brown] [name]")]
#[example("grey tank")]
#[example("white snowball")]
#[example("brown mrap")]
async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let emote = ReactionType::Unicode("✅".to_string());

    let reaction = collectors::wait_for_reaction(&ctx, emote, &msg, "Before you can create your cat and begin your adventure we have some basic rules. By reacting you are agreeing to the following rules.\n1. No alternate accounts are allowed. You are allowed one character.\n2. No trading in game items for anything outside this bot.\n3. Dont abuse any bugs or exploits you find and report them immediately.".to_string()).await?;

    if !reaction {
        msg.channel_id.say(&ctx.http, "`Timed out...`").await?;
        return Ok(());
    }

    let color = args.single::<Color>();

    if color.is_err() {
        msg.channel_id
            .say(
                &ctx.http,
                "`Invalid color, valid are grey, white and brown.`",
            )
            .await?;

        return Ok(());
    }

    let name = args.rest();

    if name.len() > 32 || name.len() < 3 {
        msg.channel_id
            .say(
                &ctx.http,
                "`Invalid name, make sure its more than 3 characters but less than 32.`",
            )
            .await?;

        return Ok(());
    }

    let heterochromia = rand::thread_rng().gen_bool(1.0 / 100.0);

    let res = Cat::add_cat(
        &db.pool,
        msg.author.id.0 as i64,
        name,
        color.unwrap(),
        heterochromia,
    )
    .await;

    match res {
        Ok(c) => {
            let formatted = format!(
                "```On this day the cat named {} was born!\nColor: {}\nHeterochromia: {}\nTreats: {}```",
                c.name,
                c.color,
                if c.heterochromia { "yes" } else { "no" },
                c.treats
            );

            let img = images::get_cat_picture(&c, false);
            let cat = images::encode(&img)?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(formatted)
                            .image("attachment://cat.png")
                            .color(constants::CONFIG.bot.color)
                    });
                    m.add_file((cat.as_slice(), "cat.png"))
                })
                .await?;
        }
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "`Please make sure the name is more than 3 characters but less than 32. Otherwise the name might be taken.`")
                .await?;
        }
    };

    Ok(())
}
