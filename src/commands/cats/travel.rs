use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::*,
};

use crate::{
    database::ConnectionPool,
    models::cat::{Area, Cat},
    utils::checks::*,
};

/*

We need our old scheduler here, travelling should take X amount of time

*/

#[command]
#[checks(HasCat)]
#[min_args(1)]
#[description("Travel to a new location. Your location determines what activities you can do:\n```Battle: where you can battle other cats\nReplenish: allows you to eat or drink giving you a quick bonus to a stat\nRest: let your cat sleep and has the chance of collecting loot while asleep```")]
#[usage("[battle|replenish|rest]")]
#[example("battle")]
#[example("replenish")]
#[example("rest")]
async fn travel(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let area = match args.single::<Area>() {
        Ok(a) => a,
        Err(_) => {
            msg.channel_id
                .say(
                    &ctx.http,
                    "`Invalid area, valid are battle, replenish and rest.`",
                )
                .await?;
            return Ok(());
        }
    };

    let res = Cat::get_cat(&db.pool, msg.author.id.0 as i64).await;

    match res {
        Ok(mut c) => {
            if c.area == area {
                msg.channel_id
                    .say(&ctx.http, "`Youre already at this area.`")
                    .await?;

                return Ok(());
            }

            c.area = area;

            if Cat::update_cat(&db.pool, msg.author.id.0 as i64, &c)
                .await
                .is_err()
            {
                msg.channel_id
                    .say(&ctx.http, "`Unknown database error.`")
                    .await?;

                return Ok(());
            };

            msg.channel_id.say(&ctx.http, "`Travelled!`").await?;
        }
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "`Unknown database error.`")
                .await?;
        }
    };

    Ok(())
}
