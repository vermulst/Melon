use pumpkin_plugin_api::{
    command::{Arg, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    server::Player,
    text::TextComponent,
    wit::pumpkin::plugin::command::CommandError,
    Result, Server,
};

pub fn send_ping_message(sender: &CommandSender, target: &Player, is_self: bool) {
    let ping = target.get_ping();

    let color_code = if ping < 100 {
        "&a"
    } else if ping < 200 {
        "&e"
    } else {
        "&c"
    };

    let message = if is_self {
        format!("&aYour ping is {color_code}{ping}ms&a.&r")
    } else {
        let name = target.get_name();
        format!("&a{name}'s ping is {color_code}{ping}ms&a.&r")
    };

    sender.send_system_message(
        TextComponent::from_legacy_string_with_code(&message, '&')
    );
}

pub struct PingHandler;

impl CommandHandler for PingHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        match args.get_value("target") {
            Arg::Players(players) => {
                if players.is_empty() {
                    return Err(CommandError::CommandFailed(
                        TextComponent::from_legacy_string_with_code("&cNo target player found.", '&'),
                    ));
                }

                for target in players {
                    send_ping_message(&sender, &target, false);
                }

                Ok(0)
            }
            _ => {
                if let Some(player) = sender.as_player() {
                    send_ping_message(&sender, &player, true);
                    Ok(0)
                } else {
                    Err(CommandError::CommandFailed(
                        TextComponent::from_legacy_string_with_code(
                            "&cConsole must specify a target player.",
                            '&',
                        ),
                    ))
                }
            }
        }
    }
}