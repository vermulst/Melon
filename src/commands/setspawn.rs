use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;
use crate::config::{SpawnPoint, CONFIG, save};

pub struct SetSpawnHandler;

impl CommandHandler for SetSpawnHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("setspawn")?;
        let world = player.get_world();
        let world_name = world.get_name();
        let pos = player.get_position();

        let point = SpawnPoint {
            x: pos.0,
            y: pos.1,
            z: pos.2,
            yaw: player.get_yaw(),
            pitch: player.get_pitch(),
        };

        {
            let mut cfg = CONFIG.lock().unwrap();
            cfg.spawn_points.insert(world_name.clone(), point);
        }

        if let Some(folder) = crate::data_folder() {
            save(&folder);
        }

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                &format!(
                    "&aSpawn for &e{}&a set to &e{:.1}, {:.1}, {:.1}&a.&r",
                    world_name, pos.0, pos.1, pos.2
                ),
                '&',
            ),
            true,
        );

        Ok(0)
    }
}