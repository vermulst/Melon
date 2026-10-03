use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

fn get_cardinal_direction(yaw: f32) -> &'static str {
    let normalized = (yaw % 360.0 + 360.0) % 360.0;
    match normalized {
        45.0..135.0 => "West",
        135.0..225.0 => "North",
        225.0..315.0 => "East",
        _ => "South",
    }
}

pub struct CompassHandler;

impl CommandHandler for CompassHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("compass")?;
        let yaw = player.get_yaw();
        let direction = get_cardinal_direction(yaw);

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                &format!("&aYou are facing &e{direction}&a (&e{yaw:.1}°&a).&r"),
                '&',
            ),
            false,
        );

        Ok(0)
    }
}