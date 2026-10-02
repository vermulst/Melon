use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    server::Player,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::CommandError;
use crate::commands::utils::ConsumedArgsExt;

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
        args.run_for_targets_or_self(&sender, "target", |target| {
            toggle_player_flight(target);
        })
    }
}