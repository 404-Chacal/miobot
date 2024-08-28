use std::collections::HashMap;

use serenity::all::{Context, Message};

use crate::commands;

pub mod run;

pub struct CommandRegistry {
    commands: HashMap<String, Box<dyn commands::Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn command_register(&mut self, name: &str, command: Box<dyn commands::Command>) {
        self.commands.insert(name.to_string(), command);
    }

    pub async fn run(&self, ctx: &Context, msg: &Message) -> Option<commands::CommandResult> {
        let command_name: &str = msg.content.split_whitespace().next().unwrap_or_default();

        if let Some(command) = self.commands.get(command_name) {
            Some(command.run(ctx, msg).await)
        } else {
            None
        }
    }
}
