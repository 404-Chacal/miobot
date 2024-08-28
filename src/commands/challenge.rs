use serenity::{
    all::{Context, Message},
    async_trait,
};

use super::{Command, CommandResult};

pub struct ChallengeCommand;

#[async_trait]
impl Command for ChallengeCommand {
    async fn run(&self, _: &Context, _: &Message) -> CommandResult {
        CommandResult::SendMessage("Not yet implemented".to_string())
    }
}
