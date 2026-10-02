use pumpkin_plugin_api::{
    command::{Arg, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    server::Player,
    text::TextComponent,
    wit::pumpkin::plugin::command::CommandError,
    Result, Server,
};
use pumpkin_plugin_api::command_wit::Number;
use crate::commands::utils::ConsumedArgsExt;

fn parse_speed_arg(args: &ConsumedArgs) -> Option<f32> {
    match args.get_value("speed") {
        Arg::Num(Ok(num)) => match num {
            Number::Float32(val) => Some(val),
            Number::Float64(val) => Some(val as f32),
            Number::Int32(val) => Some(val as f32),
            Number::Int64(val) => Some(val as f32),
            _ => None,
        },
        Arg::Simple(s) => s.parse::<f32>().ok(),
        _ => None,
    }
}

pub fn set_player_fly_speed(player: &mut Player, speed: f32) {
    let clamped_speed = speed.clamp(1.0, 20.0);
    
    let scaled_speed = (clamped_speed * 0.05).min(1.0);

    player.set_fly_speed(scaled_speed);

    player.send_system_message(
        TextComponent::from_legacy_string_with_code(
            &format!("&aFly speed set to &e{clamped_speed:.1}&a.&r"),
            '&',
        ),
        true,
    );
}

pub struct FlySpeedHandler;

impl CommandHandler for FlySpeedHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let speed = match parse_speed_arg(&args) {
            Some(s) => s,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cInvalid speed specified. Please provide a number between 1 and 20.",
                        '&',
                    ),
                ));
            }
        };

        args.run_for_targets_or_self(&sender, "target", |target| {
            set_player_fly_speed(target, speed);
        })
    }
}