use pumpkin_plugin_api::{
    command::{Arg, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    server::Player,
    text::TextComponent,
    wit::pumpkin::plugin::command::CommandError,
    Result, Server,
};
use crate::commands::utils::ConsumedArgsExt;

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
        let sender_id = sender.as_player().map(|p| p.get_id().to_string());
        args.run_for_targets_or_self(&sender, "target", |target| {
            let is_self = sender_id.as_deref() == Some(&target.get_id().to_string());
            send_ping_message(&sender, target, is_self);
        })
    }
}