use chrono::Utc;
use serenity::{
    prelude::Context,
    model::{
        channel::Message,
    },
    framework::standard::{
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
            e.description("**>help** | Display commands.\n**>rules** | Display rules.\n\n**__Staff__**\n**>kick <user> <reason>** | Kicks <user>.\n**>ban <user> <reason>** | Bans <user>.\n**>clear <num>** | Clears <num> messages in a channel.")
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