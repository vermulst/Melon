use pumpkin_plugin_api::command::{Arg, CommandError, CommandSender, ConsumedArgs};
use pumpkin_plugin_api::{ItemStack, Player};
use pumpkin_plugin_api::common::Hand;
use pumpkin_plugin_api::text::TextComponent;

pub trait CommandSenderExt {
    fn require_player(&self, command_name: &str) -> Result<pumpkin_plugin_api::server::Player, CommandError>;
}

impl CommandSenderExt for CommandSender {
    fn require_player(&self, command_name: &str) -> Result<pumpkin_plugin_api::server::Player, CommandError> {
        self.as_player().ok_or_else(|| {
            CommandError::CommandFailed(TextComponent::from_legacy_string_with_code(
                &format!("&cConsole cannot execute /{command_name}."),
                '&',
            ))
        })
    }
}


pub trait ConsumedArgsExt {
    fn run_for_targets_or_self<F>(
        &self,
        sender: &CommandSender,
        target_arg_name: &str,
        action: F,
    ) -> Result<i32, CommandError>
    where
        F: FnMut(&mut Player);

    fn require_target_player(&self) -> Result<Player, CommandError>;
}

impl ConsumedArgsExt for ConsumedArgs {
    fn run_for_targets_or_self<F>(
        &self,
        sender: &CommandSender,
        target_arg_name: &str,
        mut action: F,
    ) -> Result<i32, CommandError>
    where
        F: FnMut(&mut Player),
    {
        match self.get_value(target_arg_name) {
            Arg::Players(players) => {
                if players.is_empty() {
                    return Err(CommandError::CommandFailed(TextComponent::text("No target player found.")));
                }

                for mut target in players {
                    action(&mut target);
                }

                Ok(0)
            }
            _ => {
                if let Some(mut player) = sender.as_player() {
                    action(&mut player);
                    Ok(0)
                } else {
                    Err(CommandError::CommandFailed(TextComponent::text(
                        "Console must specify a target player.",
                    )))
                }
            }
        }
    }

    fn require_target_player(&self) -> Result<Player, CommandError> {
        match self.get_value("target") {
            Arg::Players(players) => players.into_iter().next().ok_or_else(|| {
                CommandError::CommandFailed(TextComponent::from_legacy_string_with_code(
                    "&cNo target player found.",
                    '&',
                ))
            }),
            _ => Err(CommandError::CommandFailed(
                TextComponent::from_legacy_string_with_code("&cUsage: /invsee <player>", '&'),
            )),
        }
    }
}

pub trait PlayerExt {
    fn require_main_hand(&mut self) -> Result<ItemStack, CommandError>;
}

impl PlayerExt for Player {
    fn require_main_hand(&mut self) -> Result<ItemStack, CommandError> {
        self.get_item_in_hand(Hand::Right).ok_or_else(|| {
            CommandError::CommandFailed(TextComponent::from_legacy_string_with_code(
                "&cYou must be holding an item in your main hand.",
                '&',
            ))
        })
    }
}