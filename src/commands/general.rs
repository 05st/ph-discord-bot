use chrono::Utc;
use serenity::{
    prelude::Context,
    model::{
        channel::Message,
    },
    framework::standard::{
        Args,
        CommandResult,
        macros::command,
    },
};

#[command]
#[aliases("commands", "cmds")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.author.direct_message(ctx, |m| {
        m.embed(|e| {
            e.title("Help");
            e.description("**>help** | Display commands.\n**>rules** | Display rules.\n**>toggle <role_name>** | Toggle role.\n\n**__Staff__**\n**>kick <user> <reason>** | Kicks <user>.\n**>ban <user> <reason>** | Bans <user>.\n**>clear <num>** | Clears <num> messages in a channel.")
        })
    }).await?;
    msg.delete(ctx).await?;
    Ok(())
}

#[command]
async fn rules(ctx: &Context, msg: &Message) -> CommandResult {
    msg.author.direct_message(ctx, |m| {
        m.embed(|e| {
            e.title("Rules");
            e.description("**1)** Be nice.\n**2)** Use common sense.\n**3)** If we suspect you are under 13, you will be kicked.\n**4)** Moderators are not allowed to perform actions without reason. DM the owner if you were unfairly moderated.");
            e.footer(|f| {
                f.text(format!("Rules as of {}", Utc::today()))
            })
        })
    }).await?;
    msg.delete(ctx).await?;
    Ok(())
}

#[command]
#[aliases("role")]
#[min_args(1)]
async fn toggle(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let role_name = args.rest();

    println!("ok starting");
    if let Some(guild) = msg.guild(&ctx).await {
        println!("ok found guild");
        if let Ok(mut member) = guild.member(ctx, msg.author.id).await {
            println!("ok found member");
            for (id, role) in &guild.roles {
                println!("ok looping through");
                if role.name == role_name {
                    println!("ok found role");
                    if let Ok(does) = msg.author.has_role(&ctx, guild.id, role).await {
                        println!("ok(does)");
                        if does {
                            member.remove_role(&ctx, id).await?;
                        } else {
                            member.add_role(&ctx, id).await?;
                        }
                    }
                }
            }
        }
    }
    msg.delete(ctx).await?;
    Ok(())
}