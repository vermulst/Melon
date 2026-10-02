use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    server::Player,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::CommandError;
use crate::commands::utils::ConsumedArgsExt;

pub fn heal_player(player: &mut Player) {
    player.set_health(20.0);
    player.set_food_level(20);
    player.send_system_message(
        TextComponent::text("§aYou have been healed!§r"),
        true,
    );
}

pub struct HealHandler;

impl CommandHandler for HealHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        args.run_for_targets_or_self(&sender, "target", |target| {
            heal_player(target);
        })
    }
}