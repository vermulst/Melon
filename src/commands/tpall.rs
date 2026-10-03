use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

pub struct TpAllHandler;

impl CommandHandler for TpAllHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("tpall")?;
        let sender_id = player.get_id();
        let world = player.get_world();
        let pos = player.get_position();
        let yaw = player.get_yaw();
        let pitch = player.get_pitch();

        let mut count = 0usize;
        for mut target in server.get_all_players() {
            if target.get_id().to_string() == sender_id.to_string() {
                continue;
            }
            target.teleport(pos, Some(yaw), Some(pitch), player.get_world());
            target.send_system_message(
                TextComponent::from_legacy_string_with_code(
                    &format!("&aYou were teleported to &e{}.&r", player.get_name()),
                    '&',
                ),
                false,
            );
            count += 1;
        }

        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&aTeleported &e{count}&a player(s) to you.&r"),
            '&',
        ));

        Ok(0)
    }
}