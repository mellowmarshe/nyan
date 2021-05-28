use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

use crate::{database::ConnectionPool, models::cat::Cat};

#[check]
#[name(HasCat)]
pub async fn has_cat(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let res = Cat::get_cat(&db.pool, msg.author.id.0 as i64).await;

    match res {
        Ok(_) => Ok(()),
        Err(_) => Err(Reason::User(
            "You need a cat to use this command.".to_string(),
        )),
    }
}

#[check]
#[name(DoesntHaveCat)]
pub async fn doesnt_have_cat(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let db = data.get::<ConnectionPool>().unwrap();

    let res = Cat::get_cat(&db.pool, msg.author.id.0 as i64).await;

    match res {
        Ok(_) => Err(Reason::User("You already have a cat.".to_string())),
        Err(_) => Ok(()),
    }
}
