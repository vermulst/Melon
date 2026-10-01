use crate::commands::teleport::{BackLocation, TELEPORT_MANAGER};
use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    wit::pumpkin::plugin::command::CommandError,
    Result, Server,
};

pub struct BackHandler;

impl CommandHandler for BackHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = match sender.as_player() {
            Some(p) => p,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cConsole cannot execute /back.",
                        '&',
                    ),
                ));
            }
        };

        let uuid = player.get_id();

        let previous_loc = match TELEPORT_MANAGER.get_back_location(&uuid.to_string()) {
            Some(loc) => loc,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cYou have no previous location to return to.",
                        '&',
                    ),
                ));
            }
        };

        let current_pos = player.get_position();
        let current_yaw = player.get_yaw();
        let current_pitch = player.get_pitch();
        let current_world_name = player.get_world().get_name();

        TELEPORT_MANAGER.set_back_location(
            uuid.to_string(),
            BackLocation {
                position: current_pos,
                yaw: current_yaw,
                pitch: current_pitch,
                world_name: current_world_name,
            },
        );

        let target_world = server.get_world_by_name(&previous_loc.world_name);

        player.teleport(
            previous_loc.position,
            Some(previous_loc.yaw),
            Some(previous_loc.pitch),
            target_world.unwrap(),
        );

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                "&aTeleported to your previous location.&r",
                '&',
            ),
            true,
        );

        Ok(0)
    }
}