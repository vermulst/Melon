use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    server::Player,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::CommandError;

pub fn toggle_player_flight(player: &mut Player) {
    let flying = !player.is_flying();
    player.set_flying(flying);

    // needed until flight is fixed in pumpkin-mc
    if flying {
        let mut pos = player.get_position();
        pos.1 += 1.0;
        player.teleport(pos, Some(player.get_yaw()), Some(player.get_pitch()), player.get_world());
    }

    let status = if flying { "§aenabled" } else { "§cdisabled" };
    player.send_system_message(
        TextComponent::text(format!("Flight mode {}§r.", status).as_str()),
        true,
    );
}

pub struct FlyHandler;

impl CommandHandler for FlyHandler {
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
                    toggle_player_flight(&mut target);
                }

                Ok(0)
            }
            _ => {
                if let Some(mut player) = sender.as_player() {
                    toggle_player_flight(&mut player);
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