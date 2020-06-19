use chrono::Utc;
use serenity::{
    prelude::Context,
    model::{
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

#[command]
#[aliases("commands", "cmds")]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.author.direct_message(ctx, |m| {
        m.embed(|e| {
            e.title("Help");
            e.description("**>help** | Display commands.\n**>rules** | Display rules.\n\n**__Staff__**\n**>kick** | Kicks user.\n**>ban** | Bans user.")
        })
    }).await?;
    msg.delete(&ctx.http).await?;
    Ok(())
}

#[command]
async fn rules(ctx: &Context, msg: &Message) -> CommandResult {
    msg.author.direct_message(ctx, |m| {
        m.embed(|e| {
            e.title("Rules");
            e.description("**1)** Be nice.\n**2)** Use common sense.\n**3)** If we suspect you are under 13, you will be banned.");
            e.footer(|f| {
                f.text(format!("Rules as of {}", Utc::today()))
            })
        })
    }).await?;
    msg.delete(&ctx.http).await?;
    Ok(())
}