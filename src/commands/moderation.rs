use serenity::{
    prelude::Context,
    model::{
        guild::Member.
        channel::Message,
        Permissions,
    },
    client::bridge::gateway::ShardId,
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};

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

async fn log_moderation() {
    
}

#[command]
#[required_permissions(KICK_MEMBERS)]
#[min_args(2)]
async fn kick(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let member = parse_member(&ctx, &msg, args.single_quoted::<String>()?).await;
    let reason = args.remains().unwrap();

    match member {
        Ok(m) => {
            m.kick_with_reason(ctx, reason);
            log_moderation();
        },
        Err(why) => println!("Failed to kick member: {:?}", why),
    }

    Ok(())
}