use std::sync::Arc;

use log::{error, info};
use serde::Deserialize;
use serenity::all::{Http, ShardMessenger};
use serenity::futures::StreamExt;
use serenity::prelude::*;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Action, Notification, RecordId, Surreal};

mod channel_config;
mod found_cosmetic_event;

// User data, which is stored and accessible in all command invocations
pub struct Data {
    db: Surreal<surrealdb::engine::remote::ws::Client>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    simple_logger::SimpleLogger::new().env().init().unwrap();

    // surreal db

    let Ok(db) = Surreal::new::<Ws>("localhost:8000").await else {
        error!("Failed to connect to surrealdb server");
        return;
    };

    let Ok(password) = dotenv::var("DATABASE_PASSWORD") else {
        error!("DATABASE_PASSWORD not in .env");
        return;
    };

    let Ok(_) = db
        .signin(Root {
            username: "root",
            password: &password,
        })
        .await
    else {
        error!("Failed to sign into database");
        return;
    };

    let Ok(_) = db.use_ns("cosmetics").use_db("cosmetics").await else {
        error!("Failed to use namespace");
        return;
    };

    info!("Logged into surrealdb server");

    // discord

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                channel_config::set_ping_channel(),
                channel_config::get_ping_channel(),
                channel_config::clear_ping_channel(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            tokio::spawn(watch_for_found_cosmetics(
                ctx.http.clone(),
                ctx.shard.clone(),
                db.clone(),
            ));

            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { db })
            })
        })
        .build();

    let Ok(token) = dotenv::var("BOT_TOKEN") else {
        error!("BOT_TOKEN not in .env file");
        return;
    };

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let Ok(mut client) = Client::builder(token, intents)
        // .event_handler(Handler)
        .framework(framework)
        .await
    else {
        error!("Error creating client");
        return;
    };

    info!("Logged into discord");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    // This is our custom error handler
    // They are many errors that can occur, so we only handle the ones we want to customize
    // and forward the rest to the default handler
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

fn parse_numerical_record_key(record: &RecordId) -> String {
    let key = record.key().to_string();
    let mut iter = key.chars();
    iter.next_back();
    iter.next();
    iter.collect::<String>()
}

async fn watch_for_found_cosmetics(
    http: Arc<Http>,
    shard: ShardMessenger,
    db: Surreal<surrealdb::engine::remote::ws::Client>,
) {
    #[derive(Debug, Deserialize)]
    struct CosmeticFound {
        #[serde(rename = "foundUser")]
        user_record: RecordId,
        #[serde(rename = "foundCosmetic")]
        cosmetic_record: RecordId,
    }

    let mut response = db
        .query("LIVE SELECT foundUser, foundCosmetic FROM cosmeticFound;")
        .await
        .unwrap();

    let mut stream = response.stream::<Notification<CosmeticFound>>(0).unwrap();

    while let Some(Ok(Notification { action, data, .. })) = stream.next().await {
        if let Action::Create = action {
            tokio::spawn(found_cosmetic_event::found_cosmetic_event(
                http.clone(),
                shard.clone(),
                db.clone(),
                data.user_record,
                data.cosmetic_record,
            ));
        }
    }
}
