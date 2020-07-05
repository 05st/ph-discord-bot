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
        guild::{
            Member,
        },
        id::{
            GuildId,
            RoleId,
        },
    },
    prelude::{
        Context,
        EventHandler,
    },
};

// Constants
const PREFIX: &str = ">";
const DEFAULT_ROLES: [RoleId; 1] = [RoleId(729152819742900276)];

// Frameworks
#[group("General")]
#[commands(help, rules, toggle)]
struct General;

#[group("Moderation")]
#[commands(kick, ban, clear)]
struct Moderation;

// EventHandler
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        ctx.set_activity(Activity::listening(format!("{}help", PREFIX).as_str())).await;
    }

    async fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, mut member: Member) {
        let msg = member.user.direct_message(&ctx, |m| {
            m.content("Welcome to the **Programming Hub** discord server!\nPlease make sure to read over the rules by typing **>rules**.")
        }).await;
        if let Err(why) = msg { println!("Error sending message: {:?}", why); }

        let res = member.add_roles(&ctx, &DEFAULT_ROLES).await;
        if let Err(why) = res { println!("Error adding default roles: {:?}", why); }
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