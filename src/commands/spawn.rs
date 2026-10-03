use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;
use crate::config::CONFIG;

pub struct SpawnHandler;

impl CommandHandler for SpawnHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = sender.require_player("spawn")?;
        let world = player.get_world();
        let world_name = world.get_name();

        let custom = CONFIG
            .lock()
            .unwrap()
            .spawn_points
            .get(&world_name)
            .copied();

        if let Some(sp) = custom {
            player.teleport(
                (sp.x, sp.y, sp.z),
                Some(sp.yaw),
                Some(sp.pitch),
                world,
            );
        } else {
            let loc = world.get_spawn_location();
            let p = loc.pos;
            player.teleport(
                (p.x as f64, p.y as f64, p.z as f64),
                Some(loc.yaw),
                Some(loc.pitch),
                world,
            );
        }

        player.send_system_message(
            TextComponent::from_legacy_string_with_code("&aTeleported to spawn.&r", '&'),
            false,
        );

        Ok(0)
    }
}