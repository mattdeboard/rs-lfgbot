#![allow(unused_attributes)]
#[macro_use]
extern crate log;
extern crate diesel;
extern crate serde_derive;
extern crate serenity;

pub mod db;

use env_logger;
use serenity::framework::standard::help_commands;
use serenity::framework::StandardFramework;
use serenity::http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::id::UserId;
use serenity::prelude::*;
use std::collections::HashSet;
use std::env;

struct Handler;

impl EventHandler for Handler {
  fn ready(&self, _: Context, ready: Ready) {
    info!("Connected as {}", ready.user.name);
  }

  fn resume(&self, _: Context, _: ResumedEvent) {
    info!("Resumed");
  }
}

fn main() {
  env_logger::init().expect("Failed to initialize env_logger");
  let token =
    env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
  let mut client = Client::new(&token, Handler).expect("Err creating client");

  {
    let mut data = client.data.lock();
    let db_pool = db::init_pool(None);
    data.insert::<db::Pool>(db_pool);
  }

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.owners(bot_owners()).prefix("~"))
      .help(help_commands::with_embeds),
  )
}

fn bot_owners() -> HashSet<UserId> {
  match http::get_current_application_info() {
    Ok(info) => {
      let mut set = HashSet::new();
      set.insert(info.owner.id);
      set
    }
    Err(why) => panic!("Couldn't get application info: {:?}", why),
  }
}
