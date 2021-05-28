mod commands;
mod config;
mod constants;
mod database;
mod events;
mod keys;
mod models;
mod utils;

use std::{error::Error};

use serenity::{framework::StandardFramework, Client};
use tracing::{debug, info};

use crate::database::ConnectionPool;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    std::env::set_var("RUST_LOG", "info,sqlx::query=error");
    tracing_subscriber::fmt::init();

    let _guard = sentry::init((
        constants::CONFIG.sentry.url.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            environment: Some(constants::CONFIG.sentry.environment.clone().into()),
            auto_session_tracking: true,
            ..Default::default()
        },
    ));

    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix(&constants::CONFIG.bot.prefix)
                .owners(constants::CONFIG.bot.owners.clone())
        })
        .prefix_only(events::on_prefix_only)
        .before(events::before)
        .after(events::after)
        .unrecognised_command(events::unrecognised_command)
        .normal_message(events::normal_message)
        .on_dispatch_error(events::on_dispatch_error)
        .group(&commands::cats::CATS_GROUP)
        .group(&commands::misc::MISC_GROUP)
        .group(&commands::treats::TREATS_GROUP)
        .help(&commands::misc::HELP);

    let mut client = Client::builder(&constants::CONFIG.bot.token)
        .event_handler(events::Handler)
        .framework(framework)
        .await?;

    info!("Connecting to database...");

    let database = ConnectionPool::new().await?;

    debug!("Running migrations...");

    database.migrations().await?;

    {
        let mut data = client.data.write().await;

        data.insert::<ConnectionPool>(database);
    }

    info!("Starting bot");

    client.start().await?;

    Ok(())
}
