use serenity::model::channel::Message;
use serenity::model::user::User;

command!(showme(ctx, msg, args) {
  let mut data = ctx.data.lock();
  showme_teams();
});

pub fn showme_teams() {
  return;
}


