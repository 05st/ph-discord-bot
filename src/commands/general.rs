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
            e.description("ok it work")
        })
    }).await?;
    msg.delete(&ctx.http).await?;
    Ok(())
}