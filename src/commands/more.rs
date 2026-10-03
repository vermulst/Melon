use pumpkin_plugin_api::command::{CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::Server;
use crate::commands::utils::{CommandSenderExt, PlayerExt};

pub struct MoreHandler;

impl CommandHandler for MoreHandler {
    fn handle(&self, sender: CommandSender, server: Server, args: ConsumedArgs) -> pumpkin_plugin_api::Result<i32, CommandError> {
        let mut player = sender.require_player("rename")?;
        let mut item = player.require_main_hand()?;

        item.set_count(item.get_max_count());
        player.set_item_in_hand(Hand::Right, Some(item));

        Ok(0)
    }
}