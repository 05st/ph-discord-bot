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
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    Ok(())
}