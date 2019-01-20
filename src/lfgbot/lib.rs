#![allow(unused_attributes)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serenity;

pub mod db;
pub mod models;
pub mod commands;

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

pub fn client_setup() {
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
      .help(help_commands::with_embeds)
      .group("Players LFG", |g| {
        g.command("showteams", |c| {
          c.desc("Show a list of teams looking for players matching your specified roles")
            .cmd(commands::showme::showme)
        })
      })
      .group("Teams LFT", |g| {
        g.command("showplayers", |c| {
          c.desc("Show a list of players LFT matching your specified needs")
            .cmd(commands::showme::showme)
        })
      }),
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
