use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
    prelude::*,
};

use crate::{database::ConnectionPool, models::cat::Cat, utils::checks::*};

#[command]
#[checks(HasCat)]
#[aliases("balance")]
#[description("Check how many treats you have")]
async fn treats(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let res = Cat::get_cat(&db.pool, msg.author.id.0 as i64).await;

    match res {
        Ok(c) => {
            msg.channel_id
                .say(&ctx.http, format!("`You have {} treats.`", c.treats))
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
