use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use pumpkin_plugin_api::{
    command::{Arg, CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};
use pumpkin_plugin_api::command::{CommandSuggestion, CommandSuggestions, SuggestionRequest};
use pumpkin_plugin_api::commands::CommandSuggestionHandler;

pub enum Mute {
    Permanent,
    Until(Instant),
}

impl Mute {
    fn is_active(&self) -> bool {
        match self {
            Mute::Permanent => true,
            Mute::Until(t) => *t > Instant::now(),
        }
    }
}

pub enum MuteState {
    NotMuted,
    Permanent,
    Timed(Duration),
}

pub static MUTED: LazyLock<Mutex<HashMap<String, Mute>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Returns the current mute state for a UUID, lazily pruning expired entries.
pub fn get_mute_state(uuid: &str) -> MuteState {
    let mut map = MUTED.lock().unwrap();
    match map.get(uuid) {
        Some(Mute::Permanent) => MuteState::Permanent,
        Some(Mute::Until(t)) => {
            let now = Instant::now();
            if *t > now {
                MuteState::Timed(*t - now)
            } else {
                map.remove(uuid);
                MuteState::NotMuted
            }
        }
        None => MuteState::NotMuted,
    }
}

/// Parse durations like `30s`, `5m`, `1h`, `2h`, `1d`. Bare numbers are minutes.
fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim().to_lowercase();
    if s.is_empty() {
        return None;
    }

    let (num_str, unit_secs): (&str, u64) = if let Some(stripped) = s.strip_suffix('s') {
        (stripped, 1)
    } else if let Some(stripped) = s.strip_suffix('m') {
        (stripped, 60)
    } else if let Some(stripped) = s.strip_suffix('h') {
        (stripped, 3600)
    } else if let Some(stripped) = s.strip_suffix('d') {
        (stripped, 86_400)
    } else {
        (s.as_str(), 60)
    };

    let n: u64 = num_str.trim().parse().ok()?;
    if n == 0 {
        return None;
    }
    Some(Duration::from_secs(n.saturating_mul(unit_secs)))
}

pub fn format_duration(d: Duration) -> String {
    let secs = d.as_secs();
    let days = secs / 86_400;
    let hours = (secs % 86_400) / 3_600;
    let minutes = (secs % 3_600) / 60;
    let seconds = secs % 60;

    let mut parts = Vec::new();
    if days > 0 { parts.push(format!("{days}d")); }
    if hours > 0 { parts.push(format!("{hours}h")); }
    if minutes > 0 { parts.push(format!("{minutes}m")); }
    if seconds > 0 || parts.is_empty() { parts.push(format!("{seconds}s")); }
    parts.join(" ")
}

pub struct MuteHandler;

impl CommandHandler for MuteHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let players = match args.get_value("target") {
            Arg::Players(p) => p,
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /mute <player> [duration]",
                        '&',
                    ),
                ));
            }
        };

        if players.is_empty() {
            return Err(CommandError::CommandFailed(
                TextComponent::from_legacy_string_with_code(
                    "&cNo target player found.",
                    '&',
                ),
            ));
        }

        let duration = match args.get_value("duration") {
            Arg::Simple(s) if !s.trim().is_empty() => match parse_duration(&s) {
                Some(d) => Some(d),
                None => {
                    return Err(CommandError::CommandFailed(
                        TextComponent::from_legacy_string_with_code(
                            "&cInvalid duration. Examples: &e30s&c, &e5m&c, &e1h&c, &e2h&c, &e1d&c.",
                            '&',
                        ),
                    ));
                }
            },
            _ => None,
        };

        for target in players {
            let uuid = target.get_id().to_string();
            let name = target.get_name();

            let was_muted = {
                let mut map = MUTED.lock().unwrap();
                let currently = map.get(&uuid).map(|m| m.is_active()).unwrap_or(false);
                if currently {
                    map.remove(&uuid);
                    true
                } else {
                    let m = match duration {
                        Some(d) => Mute::Until(Instant::now() + d),
                        None => Mute::Permanent,
                    };
                    map.insert(uuid, m);
                    false
                }
            };

            let msg = if was_muted {
                format!("&a{name} has been unmuted.&r")
            } else {
                match duration {
                    Some(d) => format!("&a{name} has been muted for {}.&r", format_duration(d)),
                    None => format!("&a{name} has been muted permanently.&r"),
                }
            };

            sender.send_system_message(TextComponent::from_legacy_string_with_code(&msg, '&'));
        }

        Ok(0)
    }
}

pub struct DurationSuggestions;

impl CommandSuggestionHandler for DurationSuggestions {
    fn suggest(
        &self,
        _sender: CommandSender,
        _server: Server,
        request: SuggestionRequest,
    ) -> CommandSuggestions {
        const OPTIONS: &[&str] = &["5m", "15m", "30m", "1h", "2h", "6h", "1d", "7d"];

        let prefix: &str = &request.remaining;

        let values: Vec<CommandSuggestion> = OPTIONS
            .iter()
            .filter(|o| o.starts_with(prefix))
            .map(|o| CommandSuggestion {
                value: (*o).to_string(),
                tooltip: None,
            })
            .collect();

        CommandSuggestions {
            start: request.start,
            length: prefix.len() as u32,
            values,
        }
    }
}