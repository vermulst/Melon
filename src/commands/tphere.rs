use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::{CommandSenderExt, ConsumedArgsExt};

pub struct TpHereHandler;

impl CommandHandler for TpHereHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("tphere")?;
        let target = args.require_target_player()?;

        if target.get_id().to_string() == player.get_id().to_string() {
            return Err(CommandError::CommandFailed(
                TextComponent::from_legacy_string_with_code(
                    "&cYou cannot teleport yourself to yourself.",
                    '&',
                ),
            ));
        }

        let world = player.get_world();
        let mut target = target;
        target.teleport(
            player.get_position(),
            Some(player.get_yaw()),
            Some(player.get_pitch()),
            world,
        );
        target.send_system_message(
            TextComponent::from_legacy_string_with_code(
                &format!("&aYou were teleported to &e{}.&r", player.get_name()),
                '&',
            ),
            false,
        );

        Ok(0)
    }
}