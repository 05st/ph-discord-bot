use std::env;
use serenity::{
    client::Client,
    model::{
        channel::Message,
        id::{
            GuildId,
            ChannelId,
        },
        guild::Member,
        user::User,
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

const PREFIX: &str = ">";
const GUILD_ID: u64 = 722938150954991626;
const MOD_ROLE_ID: u64 = 722942925297680384;
const LOG_CHANNEL_ID: u64 = 723287482459750501;

#[derive(Debug)]
enum ModerationType {
    Kick,
    Ban,
}

fn log_moderation(author: User, ctx: Context, moderation_type: ModerationType, tag: String, reason: String) {
    let channels = GuildId(GUILD_ID).channels(&ctx);
    if let Ok(guild_channels) = channels {
        let message = guild_channels.get(&ChannelId(LOG_CHANNEL_ID)).unwrap().send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Moderation");
                e.description(format!("`{}` has performed a moderation on `{}`.\n**Type: **`{:?}`\n**Reason: **`{}`", author.tag(), tag, moderation_type, reason));
                e.footer(|f| {
                    f.text(format!("{}", Utc::now()))
                })
            })
        });
        if let Err(why) = message { println!("Error logging moderation: {:?}", why); }
    } else if let Err(why) = channels {
        println!("Error: {:?}", why);
    }
}

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(PREFIX) {
            let command: Vec<String> = String::from(&msg.content[1..]).split_ascii_whitespace().map(|s| s.to_string()).collect();

            match command[0].as_str() {
                "rules" => {
                    let message = msg.author.direct_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Rules");
                            e.description("**1)** Be nice.\n**2)** Use common sense.\n**3)** If we suspect you are under 13, you will be banned.");
                            e.footer(|f| {
                                f.text(format!("Rules as of {}", Utc::today()))
                            })
                        })
                    });
                    if let Err(why) = message { println!("Error sending message: {:?}", why); }
                    if let Err(why) = msg.delete(ctx.http) { println!("Error deleting message: {:?}", why); }
                },
                "help" => {
                    let message = msg.author.direct_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Help");
                            e.description("**>help** | Display commands.\n**>rules** | Display rules.\n\n**__Staff__**\n**>kick** | Kicks user.\n**>ban** | Bans user.")
                        })
                    });
                    if let Err(why) = message { println!("Error sending message: {:?}", why); }
                    if let Err(why) = msg.delete(ctx.http) { println!("Error deleting message: {:?}", why); }
                },
                
                "kick" => {
                    if let Ok(b) = msg.author.has_role(&ctx, GUILD_ID, MOD_ROLE_ID) {
                        if b == true {
                            if let Some(member_name) = command.get(1) {
                                if let Some(_) = command.get(2) {
                                    if member_name.starts_with("<@") && member_name.ends_with('>') {
                                        let end = member_name.find(">").unwrap_or(member_name.len());
                                        let member_id: u64 = member_name[3..end].parse().unwrap();
                                        let member = &msg.guild_id.unwrap().member(&ctx, member_id);
                                        let full_reason = &command[2..].join(" ");

                                        match member {
                                            Ok(m) => {
                                                let tag = m.distinct();
                                                let before_msg = m.user.read().direct_message(&ctx, |m| {
                                                    m.content(format!("You were kicked by `{}` for reason: `{}`", msg.author.tag(), full_reason))
                                                });
                                                if let Err(why) = before_msg { println!("Error sending before moderation message: {:?}", why); }

                                                match m.to_owned().kick(&ctx) {
                                                    Ok(_) => {
                                                        log_moderation(msg.author, ctx, ModerationType::Kick, tag, full_reason.to_owned());
                                                    },
                                                    Err(why) => println!("Error kicking user: {:?}", why),
                                                };
                                            },
                                            Err(why) => println!("Error parsing user: {:?}", why),
                                        };
                                    }
                                } else {
                                    let incorrect_msg = &msg.author.direct_message(&ctx, |m| {
                                        m.content(format!("Please specify a reason for {}.", msg.content))
                                    });
                                    if let Err(why) = incorrect_msg { println!("Error sending incorrect usage message: {:?}", why); }
                                }
                            }
                        }
                    }
                },
                "ban" => {
                    if let Ok(b) = msg.author.has_role(&ctx, GUILD_ID, MOD_ROLE_ID) {
                        if b == true {
                            if let Some(member_name) = command.get(1) {
                                if let Some(_) = command.get(2) {
                                    if member_name.starts_with("<@") && member_name.ends_with('>') {
                                        let end = member_name.find(">").unwrap_or(member_name.len());
                                        let member_id: u64 = member_name[3..end].parse().unwrap();
                                        let member = &msg.guild_id.unwrap().member(&ctx, member_id);
                                        let full_reason = &command[2..].join(" ");

                                        match member {
                                            Ok(m) => {
                                                let tag = m.distinct();
                                                let before_msg = m.user.read().direct_message(&ctx, |m| {
                                                    m.content(format!("You were banned by `{}` for reason: `{}`", msg.author.tag(), full_reason))
                                                });
                                                if let Err(why) = before_msg { println!("Error sending before moderation message: {:?}", why); }

                                                match m.to_owned().ban(&ctx, &0) {
                                                    Ok(_) => {
                                                        log_moderation(msg.author, ctx, ModerationType::Ban, tag, full_reason.to_owned());
                                                    },
                                                    Err(why) => println!("Error banning user: {:?}", why),
                                                };
                                            },
                                            Err(why) => println!("Error parsing user: {:?}", why),
                                        };
                                    }
                                } else {
                                    let incorrect_msg = &msg.author.direct_message(&ctx, |m| {
                                        m.content(format!("Please specify a reason for {}.", msg.content))
                                    });
                                    if let Err(why) = incorrect_msg { println!("Error sending incorrect usage message: {:?}", why); }
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    fn guild_member_addition(&self, ctx: Context, _guild_id: GuildId, member: Member) {
        let msg = member.user.read().direct_message(&ctx, |m| {
            m.content("Welcome to the **Programming Hub** discord server!\nPlease make sure to read over the rules by typing **>rules**.")
        });
        if let Err(why) = msg { println!("Error sending message: {:?}", why); }
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
