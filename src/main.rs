#![warn(clippy::str_to_string)]

mod commands;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use log::{error, info};
use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, time::Duration};

// Types used by all command functions
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data;

lazy_static! {
    static ref MIL_TO_ENG: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("alpha", "A");
        map.insert("bravo", "B");
        map.insert("charlie", "C");
        map.insert("delta", "D");
        map.insert("echo", "E");
        map.insert("foxtrot", "F");
        map.insert("golf", "G");
        map.insert("hotel", "H");
        map.insert("india", "I");
        map.insert("juliett", "J");
        map.insert("kilo", "K");
        map.insert("lima", "L");
        map.insert("mike", "M");
        map.insert("november", "N");
        map.insert("oscar", "O");
        map.insert("papa", "P");
        map.insert("quebec", "Q");
        map.insert("romeo", "R");
        map.insert("sierra", "S");
        map.insert("tango", "T");
        map.insert("uniform", "U");
        map.insert("victor", "V");
        map.insert("whiskey", "W");
        map.insert("xray", "X");
        map.insert("x-ray", "X");
        map.insert("yankee", "Y");
        map.insert("zulu", "Z");
        map
    };
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            error!(target: "error-logger", "Error in command `{}`: {:?}.", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                error!(target: "error-logger", "Error while handling error: {}.", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("No .env file found");
    log4rs::init_file("log4rs-config.yml", Default::default())
        .expect("Failed to initialize log4rs from config file.");
    let options = poise::FrameworkOptions {
        commands: vec![
            commands::help(),
            commands::translate(),
            commands::translate_context_menu(),
            commands::shutdown(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            // Prefix is !
            prefix: Some("!".into()),
            // Track edits in command messages for the next hour
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        skip_checks_for_owners: true,
        ..Default::default()
    };

    let framework_builder = poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var."))
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::GUILDS
                | serenity::GatewayIntents::GUILD_MESSAGES
                | serenity::GatewayIntents::DIRECT_MESSAGES
                | serenity::GatewayIntents::MESSAGE_CONTENT,
        );
    info!(target: "startup-shutdown-logger", "Starting bot.");
    framework_builder.run().await.unwrap();
}