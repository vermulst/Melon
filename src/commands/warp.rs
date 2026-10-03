use pumpkin_plugin_api::{
    command::{Arg, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::command::{CommandSuggestion, CommandSuggestions, SuggestionRequest};
use pumpkin_plugin_api::commands::CommandSuggestionHandler;
use crate::commands::utils::CommandSenderExt;
use crate::config::{SpawnPoint, CONFIG, save};

pub struct SetWarpHandler;

impl CommandHandler for SetWarpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("setwarp")?;
        let name = match args.get_value("name") {
            Arg::Simple(s) => s.to_lowercase(),
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code("&cUsage: /setwarp <name>", '&'),
                ));
            }
        };

        {
            let cfg = CONFIG.lock().unwrap();
            if cfg.warps.contains_key(&name) {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        &format!(
                            "&cA warp named &e{name}&c already exists. Use &e/delwarp {name}&c first."
                        ),
                        '&',
                    ),
                ));
            }
        }

        let pos = player.get_position();
        let point = SpawnPoint {
            x: pos.0,
            y: pos.1,
            z: pos.2,
            yaw: player.get_yaw(),
            pitch: player.get_pitch(),
        };

        let world_name = player.get_world().get_name();
        {
            let mut cfg = CONFIG.lock().unwrap();
            cfg.warps.insert(name.clone(), point);
        }
        if let Some(folder) = crate::data_folder() {
            save(&folder);
        }

        player.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&aWarp &e{name}&a set in &e{world_name}&a.&r"),
            '&',
        ), false);
        Ok(0)
    }
}

pub struct DelWarpHandler;

impl CommandHandler for DelWarpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let name = match args.get_value("name") {
            Arg::Simple(s) => s.to_lowercase(),
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code("&cUsage: /delwarp <name>", '&'),
                ));
            }
        };

        let removed = {
            let mut cfg = CONFIG.lock().unwrap();
            cfg.warps.remove(&name).is_some()
        };
        if !removed {
            return Err(CommandError::CommandFailed(
                TextComponent::from_legacy_string_with_code(
                    &format!("&cNo warp named &e{name}&c."),
                    '&',
                ),
            ));
        }
        if let Some(folder) = crate::data_folder() {
            save(&folder);
        }

        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&aWarp &e{name}&a deleted.&r"),
            '&',
        ));
        Ok(0)
    }
}

pub struct WarpHandler;

impl CommandHandler for WarpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let name = match args.get_value("name") {
            Arg::Simple(s) => s.to_lowercase(),
            _ => return list_warps(&sender),
        };

        let mut player = sender.require_player("warp")?;

        let point = {
            let cfg = CONFIG.lock().unwrap();
            cfg.warps.get(&name).copied()
        };

        let sp = match point {
            Some(p) => p,
            None => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        &format!("&cNo warp named &e{name}&c."),
                        '&',
                    ),
                ));
            }
        };

        let world = player.get_world();
        player.teleport(
            (sp.x, sp.y, sp.z),
            Some(sp.yaw),
            Some(sp.pitch),
            world,
        );

        player.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&aTeleported to warp &e{name}&a.&r"),
            '&',
        ), false);
        Ok(0)
    }
}

fn list_warps(sender: &CommandSender) -> Result<i32, CommandError> {
    let cfg = CONFIG.lock().unwrap();
    if cfg.warps.is_empty() {
        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            "&7No warps set.",
            '&',
        ));
        return Ok(0);
    }

    let mut names: Vec<&String> = cfg.warps.keys().collect();
    names.sort();

    let list = names
        .iter()
        .map(|n| format!("&e{n}"))
        .collect::<Vec<_>>()
        .join("&7, ");

    sender.send_system_message(TextComponent::from_legacy_string_with_code(
        &format!("&aWarps (&e{}&a): {list}", names.len()),
        '&',
    ));
    Ok(0)
}

pub struct WarpNameSuggestions;

impl CommandSuggestionHandler for WarpNameSuggestions {
    fn suggest(
        &self,
        _sender: CommandSender,
        _server: Server,
        request: SuggestionRequest,
    ) -> CommandSuggestions {
        let prefix: &str = &request.remaining;

        let values: Vec<CommandSuggestion> = {
            let cfg = CONFIG.lock().unwrap();
            let mut keys: Vec<&String> = cfg.warps.keys().collect();
            keys.sort();
            keys.into_iter()
                .filter(|k| k.starts_with(prefix))
                .map(|k| CommandSuggestion {
                    value: k.clone(),
                    tooltip: None,
                })
                .collect()
        };

        CommandSuggestions {
            start: request.start,
            length: prefix.len() as u32,
            values,
        }
    }
}