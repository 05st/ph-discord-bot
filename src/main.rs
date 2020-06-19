#[path = "commands/general.rs"]
mod general;
use general::*;

#[path = "commands/moderation.rs"]
mod moderation;
use moderation::*;

use std::env;
use serenity::{
    async_trait,
    client::Client,
    framework::standard::{
        macros::{
            group,
        },
        StandardFramework,
    },
    model::{
        gateway::{
            Activity,
            Ready,
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
#[commands(help, rules)]
struct General;

#[group("Moderation")]
#[commands(kick, ban)]
struct Moderation;

// EventHandler
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_activity(Activity::listening(format!("{}help", PREFIX).as_str())).await;
    }
}

// Main
#[tokio::main]
async fn main() {
    let token = env::var("PH_DISCORD_TOKEN").expect("Failed to get token from environment variable.");

    let framework = StandardFramework::new()
        .configure(|c| c
            .prefix(PREFIX)
            .case_insensitivity(true)
        )
        .group(&GENERAL_GROUP)
        .group(&MODERATION_GROUP);

    let mut client = Client::new(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Failed to create client.");
    
    if let Err(why) = client.start().await {
        println!("Error starting client: {:?}", why);
    }
}