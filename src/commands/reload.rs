use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::config;
use crate::data_folder;

pub struct ReloadHandler;

impl CommandHandler for ReloadHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let folder = match data_folder() {
            Some(f) => f,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cData folder not initialized.",
                        '&',
                    ),
                ));
            }
        };

        config::load(&folder);

        sender.send_system_message(
            TextComponent::from_legacy_string_with_code(
                "&aMelon config reloaded.&r",
                '&',
            ),
        );

        Ok(0)
    }
}