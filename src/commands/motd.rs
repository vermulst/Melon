use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    Result, Server,
};
use pumpkin_plugin_api::text::TextComponent;

pub struct MotdHandler;

impl CommandHandler for MotdHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        sender.send_system_message(TextComponent::from_legacy_string_with_code(&server.get_motd(), '&'));
        Ok(0)
    }
}