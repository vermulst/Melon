use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};

use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

pub static AFK: LazyLock<Mutex<HashSet<String>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct AfkHandler;

impl CommandHandler for AfkHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("afk")?;
        let uuid = player.get_id().to_string();

        let was_afk = {
            let mut set = AFK.lock().unwrap();
            if set.remove(&uuid) {
                true
            } else {
                set.insert(uuid);
                false
            }
        };

        let msg = if was_afk {
            "&aYou are no longer AFK.&r"
        } else {
            "&7You are now AFK.&r"
        };

        player.send_system_message(TextComponent::from_legacy_string_with_code(msg, '&'), true);
        Ok(0)
    }
}