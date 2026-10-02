use pumpkin_plugin_api::command::{Arg, CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::{ItemStackExt, Server};
use pumpkin_plugin_api::text::TextComponent;
use crate::commands::utils::{CommandSenderExt, PlayerExt};

pub struct RenameHandler;


impl CommandHandler for RenameHandler {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let mut player = sender.require_player("rename")?;
        let mut item = player.require_main_hand()?;
        let new_name = match args.get_value("name") {
            Arg::Simple(s) => s,
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /rename <name>",
                        '&',
                    ),
                ));
            }
        };
        let formatted_name = TextComponent::from_legacy_string_with_code(&new_name, '&');
        item.set_custom_name(Some(formatted_name));
        player.set_item_in_hand(Hand::Right, Some(item));
        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                "&aItem renamed successfully.&r",
                '&',
            ),
            true,
        );


        Ok(0)
    }
}