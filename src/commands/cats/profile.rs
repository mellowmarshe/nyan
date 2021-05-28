use std::{
    borrow::Cow,
    fs::File,
    io::{BufWriter, Read, Write},
    path::Path,
};

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

use crate::{
    constants,
    database::ConnectionPool,
    models::cat::Cat,
    utils::{
        checks::*,
        images::{self},
    },
};

#[command]
#[checks(HasCat)]
#[aliases("cat")]
#[description("Check your cats profile")]
async fn profile(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let res = Cat::get_cat(&db.pool, msg.author.id.0 as i64).await;

    match res {
        Ok(c) => {
            let formatted = format!(
                "```Name: {}\nColor: {}\nHeterochromia: {}\nTreats: {}\nArea: {} area\nBorn {} UTC```",
                c.name,
                c.color,
                if c.heterochromia { "yes" } else { "no" },
                c.treats,
                c.area,
                c.created
            );

            let profile = images::encode(&images::overlay_on_house(&c))?;

            msg.channel_id
                .send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.description(formatted)
                            .image("attachment://profile.png")
                            .color(constants::CONFIG.bot.color)
                    });
                    m.add_file((profile.as_slice(), "profile.png"))
                })
                .await?;
        }
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "`Unknown database error.`")
                .await?;
        }
    };

    Ok(())
}
