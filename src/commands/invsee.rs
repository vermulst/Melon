use pumpkin_plugin_api::{command::{Arg, CommandSender, ConsumedArgs}, commands::CommandHandler, gui::{Gui, Screen}, text::TextComponent, wit::pumpkin::plugin::command::CommandError, Result, Server, ItemStack};
use crate::commands::utils::{CommandSenderExt, ConsumedArgsExt};

pub struct InvseeHandler;

impl InvseeHandler {
    fn create_filler() -> ItemStack {
        let mut filler = ItemStack::new("minecraft:gray_stained_glass_pane", 1);
        filler.set_custom_name(Some(TextComponent::from_legacy_string_with_code("&r", '&')));
        filler
    }
}

impl CommandHandler for InvseeHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mut player = sender.require_player("invsee")?;
        let target_player = args.require_target_player()?;

        let title = TextComponent::from_legacy_string_with_code(
            &format!("&8Inv: &0{} (read-only)", target_player.get_name()),
            '&',
        );

        let mut gui = Gui::new(Screen::Generic9x6, title);

        let target_inv = target_player.get_inventory();

        let armor_items = [
            target_inv.get_helmet(),
            target_inv.get_chestplate(),
            target_inv.get_leggings(),
            target_inv.get_boots(),
        ];

        for (i, item_opt) in armor_items.into_iter().enumerate() {
            if let Some(item) = item_opt {
                gui.set_item(i as u32, item);
            }
        }

        if let Some(mainhand) = target_player.get_item_in_hand(pumpkin_plugin_api::common::Hand::Right) {
            gui.set_item(4, mainhand);
        }

        if let Some(offhand) = target_player.get_item_in_hand(pumpkin_plugin_api::common::Hand::Left) {
            gui.set_item(5, offhand);
        }

        for slot in 6..9 {
            gui.set_item(slot, Self::create_filler());
        }

        let armor_labels = ["&9Helmet", "&9Chestplate", "&9Leggings", "&9Boots"];
        for i in 0..4 {
            let mut label = ItemStack::new("minecraft:blue_stained_glass_pane", 1);
            label.set_custom_name(Some(TextComponent::from_legacy_string_with_code(armor_labels[i], '&')));
            gui.set_item((i + 9) as u32, label);
        }

        let selected_slot = target_player.get_selected_slot();
        let mainhand_label_text = format!("&aMainhand (slot {})", selected_slot);

        let mut mainhand_label = ItemStack::new("minecraft:green_stained_glass_pane", 1);
        mainhand_label.set_custom_name(Some(TextComponent::from_legacy_string_with_code(&mainhand_label_text, '&')));
        gui.set_item(13, mainhand_label);

        let mut offhand_label = ItemStack::new("minecraft:purple_stained_glass_pane", 1);
        offhand_label.set_custom_name(Some(TextComponent::from_legacy_string_with_code("&dOffhand", '&')));
        gui.set_item(14, offhand_label);

        for slot in 15..18 {
            gui.set_item(slot, Self::create_filler());
        }

        let main_items = target_inv.as_inventory().get_all_items();

        for (slot, item_opt) in main_items.into_iter().enumerate() {
            if let Some(item) = item_opt {
                let target_slot = match slot {
                    0..=8 => 45 + slot,
                    9..=35 => 18 + (slot - 9),
                    _ => continue,
                };
                gui.set_item(target_slot as u32, item);
            }
        }

        player.open_gui(gui);

        Ok(0)
    }
}