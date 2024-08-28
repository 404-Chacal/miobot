use crate::commands::{Command, CommandResult, CreateCommand};
use serenity::{
    all::{Context, Message},
    async_trait,
};

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    async fn run(&self, _ctx: &Context, _msg: &Message) -> CommandResult {
        CommandResult::Reply("Hey, I'm alive!".to_string())
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("A ping command")
}
