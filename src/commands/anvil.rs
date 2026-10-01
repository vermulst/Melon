use pumpkin_plugin_api::{command::{CommandSender, ConsumedArgs}, commands::CommandHandler, text::TextComponent, wit::pumpkin::plugin::command::CommandError, Result, Server, Screen};
use pumpkin_plugin_api::gui::Gui;

pub struct AnvilHandler;

impl CommandHandler for AnvilHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = match sender.as_player() {
            Some(p) => p,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cConsole cannot execute /anvil.",
                        '&',
                    ),
                ));
            }
        };

        player.open_gui(Gui::new(Screen::Anvil, TextComponent::text("Repair & Name")));
        Ok(0)
    }
}