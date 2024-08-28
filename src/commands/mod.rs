use serenity::{
    all::{Context, Message},
    async_trait,
};

pub mod challenge;
pub mod help;
pub mod ping;

pub struct CreateCommand {
    pub name: String,
    pub description: Option<String>,
}

impl CreateCommand {
    pub fn new(name: &str) -> CreateCommand {
        CreateCommand {
            name: name.to_string(),
            description: None,
        }
    }

    pub fn description(mut self, description: &str) -> CreateCommand {
        self.description = Some(description.to_string());
        self
    }
}

pub enum CommandResult {
    SendMessage(String),
    Reply(String),
}

#[async_trait]
pub trait Command: Send + Sync {
    async fn run(&self, ctx: &Context, message: &Message) -> CommandResult;
}
