use pumpkin_plugin_api::{
    command::{Arg, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

pub struct WorldHandler;

impl CommandHandler for WorldHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = sender.require_player("world")?;

        let target = match args.get_value("name") {
            Arg::Simple(s) => s,
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /world <name>",
                        '&',
                    ),
                ));
            }
        };

        let world = match server.get_world_by_name(&target) {
            Some(w) => w,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        &format!("&cNo world named &e{target}&c."),
                        '&',
                    ),
                ));
            }
        };

        let spawn = world.get_spawn_location();
        let pos = spawn.pos;
        player.teleport(
            (pos.x as f64, pos.y as f64, pos.z as f64),
            Some(spawn.yaw),
            Some(spawn.pitch),
            world,
        );

        player.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&aTeleported to &e{target}&a.&r"),
            '&',
        ), false);

        Ok(0)
    }
}