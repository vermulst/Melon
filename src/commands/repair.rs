use pumpkin_plugin_api::command::{CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::{ItemStackExt, Server};
use pumpkin_plugin_api::text::TextComponent;
use crate::commands::utils::CommandSenderExt;

pub struct RepairHandler;


//todo: durability not yet implemented in pumpkin
impl CommandHandler for RepairHandler {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let mut player = sender.require_player("repair")?;

        let mut item = match player.get_item_in_hand(Hand::Right) {
            Some(item) => item,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cYou must be holding an item in your main hand to repair it.",
                        '&',
                    ),
                ));
            }
        };

        let damage = 0;
        if damage == 0 {
            return Err(CommandError::CommandFailed(
                TextComponent::from_legacy_string_with_code(
                    "&cThis item is already fully repaired.",
                    '&',
                ),
            ));
        }


        Ok(0)
    }
}