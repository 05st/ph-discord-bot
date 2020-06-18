use std::env;
use serenity::{
    client::Client,
    model::{
        channel::Message,
        id::GuildId,
        guild::Member,
        gateway::{
            Ready,
            Activity,
        }
    },
    prelude::{
        EventHandler,
        Context
    },
};
use chrono::Utc;

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == ">rules" {
            let send_msg = msg.author.direct_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Rules");
                    e.description("**1)** Be nice.\n**2)** Use common sense.\n**3)** If we suspect you are under 13, you will be banned.");
                    e.footer(|f| {
                        f.text(format!("Rules as of {}", Utc::now()))
                    })
                })
            });

            if let Err(why) = send_msg {
                println!("Error sending message: {:?}", why);
            }

            msg.delete(ctx.http).unwrap();
        } else if msg.content == ">help" {
            let send_msg = msg.author.direct_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Help");
                    e.description("**>help** | Display this bot's commands.\n**>rules** | Display the server's rules.");
                    e
                });
                m
            });

            if let Err(why) = send_msg {
                println!("Error sending message: {:?}", why);
            }

            msg.delete(ctx.http).unwrap();
        }
    }

    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, member: Member) {
        let msg = member.user.read().direct_message(&ctx, |m| {
            m.content("Welcome to the Programming Hub discord server!\nPlease make sure to read over the rules by typing **>rules**.");
            m
        });

        if let Err(why) = msg {
            println!("Error sending message: {:?}", why);
        }
    }

    fn ready(&self, ctx: Context, _: Ready) {
        ctx.set_activity(Activity::listening(">help"));
    }
}

fn main() {
    let token = env::var("PH_DISCORD_TOKEN").expect("Failed to get token");
    let mut client = Client::new(token, Handler).expect("Error creating bot client");

    if let Err(why) = client.start() {
        println!("An error occured while running the client: {:?}", why);
    }
}
