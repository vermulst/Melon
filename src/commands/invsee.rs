use pumpkin_plugin_api::{
    command::{Arg, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    common::Hand,
    text::TextComponent,
    wit::pumpkin::plugin::command::CommandError,
    ItemStackExt, Result, Server,
};

pub struct InvseeHandler;

impl CommandHandler for InvseeHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = match sender.as_player() {
            Some(p) => p,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cConsole cannot execute /invsee.",
                        '&',
                    ),
                ));
            }
        };

        let target_arg = args.get_value("target");

        let target_player = match target_arg {
            Arg::Players(players) => {
                if let Some(target) = players.into_iter().next() {
                    target
                } else {
                    return Err(CommandError::CommandFailed(
                        TextComponent::from_legacy_string_with_code(
                            "&cNo target player found.",
                            '&',
                        ),
                    ));
                }
            }
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /invsee <player>",
                        '&',
                    ),
                ));
            }
        };

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                &format!("&6--- Inventory of &e{} &6---", target_player.get_name()),
                '&',
            ),
            false,
        );

        let inventory_items = target_player.get_inventory();

        let main_hand_opt = target_player.get_item_in_hand(Hand::Right);
        let off_hand_opt = target_player.get_item_in_hand(Hand::Left);

        let main_hand_str = main_hand_opt
            .as_ref()
            .and_then(|i| i.get_item())
            .map(|i| i.to_string().to_lowercase())
            .unwrap_or_else(|| "air".to_string());
        let main_hand_count = main_hand_opt.as_ref().map_or(0, |i| i.get_count());

        let off_hand_str = off_hand_opt
            .as_ref()
            .and_then(|i| i.get_item())
            .map(|i| i.to_string().to_lowercase())
            .unwrap_or_else(|| "air".to_string());
        let off_hand_count = off_hand_opt.as_ref().map_or(0, |i| i.get_count());

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                &format!(
                    "&eMain Hand: &f{} (x{})\n&eOff Hand: &f{} (x{})",
                    main_hand_str, main_hand_count, off_hand_str, off_hand_count
                ),
                '&',
            ),
            false,
        );

        let armor_slots = ["Helmet", "Chestplate", "Leggings", "Boots"];
        let armor_items = [
            inventory_items.get_helmet(),
            inventory_items.get_chestplate(),
            inventory_items.get_leggings(),
            inventory_items.get_boots(),
        ];

        player.send_system_message(
            TextComponent::from_legacy_string_with_code("&6Armor:", '&'),
            false,
        );

        for (index, label) in armor_slots.iter().enumerate() {
            if let Some(item) = armor_items[index].as_ref() {
                if item.get_count() > 0 {
                    let item_name = item
                        .get_item()
                        .map(|i| i.to_string().to_lowercase())
                        .unwrap_or_else(|| "unknown".to_string());
                    player.send_system_message(
                        TextComponent::from_legacy_string_with_code(
                            &format!("  &e{}: &f{} (x{})", label, item_name, item.get_count()),
                            '&',
                        ),
                        false,
                    );
                }
            }
        }

        player.send_system_message(
            TextComponent::from_legacy_string_with_code("&6Hotbar & Main Inventory:", '&'),
            false,
        );

        let mut has_items = false;
        let all_items = inventory_items.as_inventory().get_all_items();

        for (slot, item_opt) in all_items.iter().enumerate() {
            if let Some(item) = item_opt {
                if item.get_count() > 0 {
                    has_items = true;
                    let item_name = item
                        .get_item()
                        .map(|i| i.to_string().to_lowercase())
                        .unwrap_or_else(|| "unknown".to_string());
                    player.send_system_message(
                        TextComponent::from_legacy_string_with_code(
                            &format!("  &7Slot {}: &f{} (x{})", slot, item_name, item.get_count()),
                            '&',
                        ),
                        false,
                    );
                }
            }
        }

        if !has_items {
            player.send_system_message(
                TextComponent::from_legacy_string_with_code("  &7(Inventory empty)", '&'),
                false,
            );
        }

        Ok(0)
    }
}