use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};

pub struct ListHandler;

impl CommandHandler for ListHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let players = server.get_all_players();
        let count = players.len();

        if count == 0 {
            sender.send_system_message(TextComponent::from_legacy_string_with_code(
                "&aOnline players (&e0&a): &7none&r",
                '&',
            ));
            return Ok(0);
        }

        let names: Vec<String> = players
            .iter()
            .map(|p| format!("&e{}", p.get_name()))
            .collect();

        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!(
                "&aOnline players (&e{count}&a): {}",
                names.join("&7, ")
            ),
            '&',
        ));

        Ok(0)
    }
}