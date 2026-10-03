use pumpkin_plugin_api::{
    command::{Arg, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};

pub struct BroadcastHandler;

impl CommandHandler for BroadcastHandler {
    fn handle(
        &self,
        _sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let message_str = match args.get_value("message") {
            Arg::Simple(s) => s,
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /broadcast <message>",
                        '&',
                    ),
                ));
            }
        };

        let formatted = format!("&d[Broadcast]&r {message_str}");

        for mut player in server.get_all_players() {
            let component = TextComponent::from_legacy_string_with_code(&formatted, '&');
            player.send_system_message(component, false);
        }

        Ok(0)
    }
}