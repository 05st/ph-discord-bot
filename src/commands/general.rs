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

    Ok(())
}