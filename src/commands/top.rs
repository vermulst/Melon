use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    world::BlockPos,
    BlockType, BlockTypeExt, Result, Server,
};
use crate::commands::utils::CommandSenderExt;

pub struct TopHandler;

impl CommandHandler for TopHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = sender.require_player("top")?;
        let pos = player.get_position();
        let world = player.get_world();

        let block_x = pos.0.floor() as i32;
        let block_z = pos.2.floor() as i32;

        let mut target_y = None;

        for y in (-64..=318).rev() {
            let feet_block = world.get_block(BlockPos { x: block_x, y, z: block_z });
            let head_block = world.get_block(BlockPos { x: block_x, y: y + 1, z: block_z });
            let ground_block = world.get_block(BlockPos { x: block_x, y: y - 1, z: block_z });

            if !ground_block.is_block_type(BlockType::Air)
                && feet_block.is_block_type(BlockType::Air)
                && head_block.is_block_type(BlockType::Air)
            {
                target_y = Some(y as f64);
                break;
            }
        }

        let final_y = target_y.unwrap_or(319.0);

        player.teleport(
            (pos.0, final_y, pos.2),
            Some(player.get_yaw()),
            Some(player.get_pitch()),
            world,
        );

        player.send_system_message(
            TextComponent::from_legacy_string_with_code("&aTeleported to highest safe surface.&r", '&'),
            false,
        );

        Ok(0)
    }
}