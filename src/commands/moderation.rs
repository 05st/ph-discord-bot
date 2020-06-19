use chrono::Utc;
use serenity::{
    prelude::Context,
    model::{
        channel::Message,
        guild::Member,
        id::ChannelId,
        Permissions,
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};

// Constants
const LOG_CHANNEL_ID: u64 = 723287482459750501; // Where moderation logs get sent

// Moderation Type Enum
#[derive(Debug)]
enum ModerationType {
    Kick,
    Ban,
}

async fn parse_member(ctx: &Context, msg: &Message, member_name: String) -> Result<Member, String> {
    if member_name.starts_with("<@") && member_name.ends_with('>') {
        let end = member_name.find(">").unwrap_or(member_name.len());
        let member_id: u64 = member_name[3..end].parse().unwrap();
        let member = &msg.guild_id.unwrap().member(ctx, member_id).await;
        match member {
            Ok(m) => Ok(m.to_owned()),
            Err(why) => Err(why.to_string()),
        }
    } else {
        Err(String::from("Invalid user tag."))
    }
}

async fn log_moderation(ctx: &Context, msg: &Message, moderator_name: String, victim_name: String, moderation_type: ModerationType, reason: String) {
    if let Some(guild_id) = msg.guild_id {
        if let Ok(guild_channels) = guild_id.channels(&ctx).await {
            if let Some(channel) = guild_channels.get(&ChannelId(LOG_CHANNEL_ID)) {
                let message = channel.send_message(ctx, |m| {
                    m.embed(|e| {
                        e.title("Moderation");
                        e.description(format!("**Moderator:** {}\n**Victim:** {}\n**Type:** {:?}\n**Reason:** {}", moderator_name, victim_name, moderation_type, reason));
                        e.footer(|f| {
                            f.text(Utc::now())
                        })
                    })
                }).await;
                if let Err(why) = message {
                    println!("Failed to log moderation: {:?}", why);
                }
            }
        }
    }
}

#[command]
#[required_permissions(KICK_MEMBERS)]
#[min_args(2)]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member = parse_member(&ctx, &msg, args.single_quoted::<String>()?).await;
    if let Some(reason) = args.remains() {
        match member {
            Ok(m) => {
                if let Err(why) = m.kick_with_reason(ctx, reason).await {
                    println!("Failed to kick member: {:?}", why);
                } else {
                    log_moderation(&ctx, &msg, format!("{}#{} ({})", msg.author.name, msg.author.discriminator, msg.author.id), format!("{}#{} ({})", m.user.name, m.user.discriminator, m.user.id), ModerationType::Kick, reason.to_string()).await;
                }
            },
            Err(why) => println!("Failed to parse member: {:?}", why),
        }
    }
    Ok(())
}

#[command]
#[required_permissions(BAN_MEMBERS)]
#[min_args(2)]
async fn ban(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member = parse_member(&ctx, &msg, args.single_quoted::<String>()?).await;
    if let Some(reason) = args.remains() {
        match member {
            Ok(m) => {
                if let Err(why) = m.ban_with_reason(ctx, 0, reason).await {
                    println!("Failed to ban member: {:?}", why);
                } else {
                    log_moderation(&ctx, &msg, format!("{}#{} ({})", msg.author.name, msg.author.discriminator, msg.author.id), format!("{}#{} ({})", m.user.name, m.user.discriminator, m.user.id), ModerationType::Ban, reason.to_string()).await;
                }
            },
            Err(why) => println!("Failed to parse member: {:?}", why),
        }
    }
    Ok(())
}