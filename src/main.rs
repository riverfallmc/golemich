//! =GOLEMICH=

use std::{env, sync::LazyLock};
use anyhow::Result;
use handler::MessageHandler;
use serenity::{all::{ActivityData, GatewayIntents}, Client};

mod handler;
mod actions;
mod utils;

static TOKEN: LazyLock<String> = LazyLock::new(|| env::var("APPLICATION_TOKEN").unwrap());

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();

  let mut client = Client::builder(TOKEN.to_owned(), GatewayIntents::all())
    .event_handler(MessageHandler)
    .activity(ActivityData::playing("Riverfall"))
    .await?;

  client.start()
    .await?;

  Ok(())
}