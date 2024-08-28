use serenity::all::*;
use serenity::async_trait;
use serenity::model::channel::Message;

use crate::commands;
use crate::handlers::CommandRegistry;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {
        eprintln!("{}", msg.content);

        let mut registry: CommandRegistry = CommandRegistry::new();

        registry.command_register("!ping", Box::new(commands::ping::PingCommand));
        registry.command_register("!help", Box::new(commands::help::HelpCommand));

        registry.command_register(
            "!challenge",
            Box::new(commands::challenge::ChallengeCommand),
        );

        if let Some(content) = registry.run(&ctx, &msg).await {
            match content {
                commands::CommandResult::SendMessage(content) => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
                        eprintln!("Error sending message: {why:?}");
                    }
                }
                commands::CommandResult::Reply(content) => {
                    if let Err(why) = msg.reply(&ctx.http, content).await {
                        eprintln!("Error sending message: {why:?}");
                    }
                }
            }
        }
    }
}
