use log::info;
use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::gateway::ActivityData;
use serenity::async_trait;

use super::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Logged in as {}!", ready.user.tag());
        ctx.set_activity(Some(ActivityData::watching("Azumanga Daioh")));
    }
}