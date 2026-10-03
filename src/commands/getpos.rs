use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

pub struct GetPosHandler;

impl CommandHandler for GetPosHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("getpos")?;
        let pos = player.get_position();
        let yaw = player.get_yaw();
        let pitch = player.get_pitch();

        let message = format!(
            "&aPosition: &e{:.2}&a, &e{:.2}&a, &e{:.2}&a (&e{:.1}°&a / &e{:.1}°&a)&r",
            pos.0, pos.1, pos.2, yaw, pitch
        );

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(&message, '&'),
            false
        );


        Ok(0)
    }
}