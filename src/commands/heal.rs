use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    server::Player,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::CommandError;

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
        let target_arg = args.get_value("target");

        match target_arg {
            pumpkin_plugin_api::command::Arg::Players(players) => {
                if players.is_empty() {
                    return Err(CommandError::CommandFailed(TextComponent::text("No target player found.")));
                }

                for mut target in players {
                    heal_player(&mut target);
                }

                Ok(0)
            }
            _ => {
                if let Some(mut player) = sender.as_player() {
                    heal_player(&mut player);
                    Ok(0)
                } else {
                    Err(CommandError::CommandFailed(TextComponent::text(
                        "Console must specify a target player.",
                    )))
                }
            }
        }
    }
}