use crate::commands;
use crate::commands::{Command, CommandResult, CreateCommand};
use serenity::all::{Context, Message};
use serenity::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    async fn run(&self, _ctx: &Context, _msg: &Message) -> CommandResult {
        let commands: Vec<CreateCommand> =
            vec![commands::ping::register(), commands::help::register()];
        let mut output: String = String::new();
        output.push_str("___________\n");

        for command in &commands {
            let description = command
                .description
                .as_ref()
                .unwrap_or(&"No description".to_string())
                .to_string();

            let line = format!("{} - {}\n", command.name, description);
            output.push_str(&line);
        }

        CommandResult::SendMessage(output)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("A help command that lists all the commands")
}
