#[path = "commands/general.rs"]
mod general;
use general::*;

#[path = "commands/moderation.rs"]
mod moderation;
use moderation::*;

use std::env;
use serenity::{
    async_trait,
    client::{
        bridge::{
            gateway::{
                GatewayIntents,
                ShardManager,
            },
        },
        Client,
    },
    framework::standard::{
        Args,
        CommandOptions,
        DispatchError,
        HelpBehaviour,
        macros::{
            command,
            group,
        },
        StandardFramework,
    },
    model::{
        channel::{
            Message,
            Reaction,
            ReactionType,
        },
        gateway::{
            Activity,
            Ready,
        },
        id::{
            GuildId,
            ChannelId,
            UserId,
        },
    },
    prelude::{
        Context,
        EventHandler,
    },
};

// Constants
const PREFIX: &str = ">";

// Frameworks
#[group("General")]
#[commands(help)]
struct General;

#[group("Moderation")]
#[commands(kick)]
struct Moderation;

// EventHandler
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::listening(format!("{}help", PREFIX)));
    }
}

// Main
#[tokio::main]
async fn main() {
    let token = env::var("PH_DISCORD_TOKEN").expect("Failed to get token from environment variable.");
    let mut client = Client::new(token).event_handler(Handler).await;

    let framework = StandardFramework::new();
}