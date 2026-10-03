use pumpkin_plugin_api::{
    command::{Arg, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    player::CustomStatistic,
    text::TextComponent,
    Result, Server,
};
use crate::commands::utils::CommandSenderExt;

fn format_ticks(ticks: u64) -> String {
    let secs = ticks / 20;
    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let minutes = (secs % 3_600) / 60;
    let seconds = secs % 60;

    let mut parts = Vec::new();
    if days > 0    { parts.push(format!("{days}d")); }
    if hours > 0   { parts.push(format!("{hours}h")); }
    if minutes > 0 { parts.push(format!("{minutes}m")); }
    parts.push(format!("{seconds}s"));
    parts.join(" ")
}

pub struct PlaytimeHandler;

impl CommandHandler for PlaytimeHandler {
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
                        TextComponent::from_legacy_string_with_code(
                            "&cNo target player found.", '&',
                        ),
                    ));
                }
                for player in &players {
                    let ticks =
                        player.get_custom_statistic(CustomStatistic::TotalWorldTime) as u64;
                    sender.send_system_message(
                        TextComponent::from_legacy_string_with_code(
                            &format!(
                                "&a{} has played for &e{}.&r",
                                player.get_name(),
                                format_ticks(ticks),
                            ),
                            '&',
                        ),
                    );
                }
                Ok(0)
            }
            _ => {
                let player = sender.require_player("playtime")?;
                let ticks =
                    player.get_custom_statistic(CustomStatistic::TotalWorldTime) as u64;
                player.send_system_message(
                    TextComponent::from_legacy_string_with_code(
                        &format!("&aYou have played for &e{}.&r", format_ticks(ticks)),
                        '&',
                    ),
                    false,
                );
                Ok(0)
            }
        }
    }
}