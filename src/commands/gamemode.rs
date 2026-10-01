use pumpkin_plugin_api::{
    command::{CommandSender, ConsumedArgs},
    player::GameMode,
    server::Player,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::wit::pumpkin::plugin::command::CommandError;

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Mode {
    pub fn parse(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "0" | "survival" | "s" => Some(Mode::Survival),
            "1" | "creative" | "c" => Some(Mode::Creative),
            "2" | "adventure" | "a" => Some(Mode::Adventure),
            "3" | "spectator" | "sp" => Some(Mode::Spectator),
            _ => None,
        }
    }

    pub fn to_gamemode(self) -> GameMode {
        match self {
            Mode::Survival => GameMode::Survival,
            Mode::Creative => GameMode::Creative,
            Mode::Adventure => GameMode::Adventure,
            Mode::Spectator => GameMode::Spectator,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Mode::Survival => "Survival",
            Mode::Creative => "Creative",
            Mode::Adventure => "Adventure",
            Mode::Spectator => "Spectator",
        }
    }
}

pub fn set_player_gamemode(player: &mut Player, mode: Mode) {
    player.set_gamemode(mode.to_gamemode());
    player.send_system_message(
        TextComponent::text(format!("Set gamemode to §a{}§r.", mode.name()).as_str()),
        true,
    );
}

pub struct FixedGamemodeHandler {
    pub mode: Mode,
}

impl CommandHandler for FixedGamemodeHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let target_arg = args.get_value("target");

        match target_arg {
            pumpkin_plugin_api::command::Arg::Players(players) => {
                if players.is_empty() {
                    return Err(CommandError::CommandFailed(TextComponent::text("No target player found.")));
                }

                for mut target in players {
                    set_player_gamemode(&mut target, self.mode);
                }

                Ok(0)
            }
            _ => {
                if let Some(mut player) = sender.as_player() {
                    set_player_gamemode(&mut player, self.mode);
                    Ok(0)
                } else {
                    Err(CommandError::CommandFailed(TextComponent::text(
                        "Console must specify a target player.",
                    )))
                }
            }
        }
    }
}

pub struct DynamicGamemodeHandler;

impl CommandHandler for DynamicGamemodeHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let mode_str = match args.get_value("mode") {
            pumpkin_plugin_api::command::Arg::Simple(s) => s,
            _ => return Err(CommandError::CommandFailed(TextComponent::text("Invalid mode argument."))),
        };

        let mode = match Mode::parse(&mode_str) {
            Some(m) => m,
            None => return Err(CommandError::CommandFailed(TextComponent::text("Invalid mode. Use 0-3, survival, creative, adventure, or spectator."))),
        };

        let target_arg = args.get_value("target");

        match target_arg {
            pumpkin_plugin_api::command::Arg::Players(players) => {
                if players.is_empty() {
                    return Err(CommandError::CommandFailed(TextComponent::text("No target player found.")));
                }

                for mut target in players {
                    set_player_gamemode(&mut target, mode);
                }

                Ok(0)
            }
            _ => {
                if let Some(mut player) = sender.as_player() {
                    set_player_gamemode(&mut player, mode);
                    Ok(0)
                } else {
                    Err(CommandError::CommandFailed(TextComponent::text(
                        "Console must specify a target player.",
                    )))
                }
            }
        }
    }
}