use pumpkin_plugin_api::{
    events::{EventData, EventHandler, PlayerChatEvent},
    text::TextComponent,
    Server,
};
use crate::commands::mute::{format_duration, get_mute_state, MuteState};

pub struct MuteFilter;

impl EventHandler<PlayerChatEvent> for MuteFilter {
    fn handle(
        &self,
        _server: Server,
        mut event: EventData<PlayerChatEvent>,
    ) -> EventData<PlayerChatEvent> {
        let uuid = event.player.get_id().to_string();

        let msg = match get_mute_state(&uuid) {
            MuteState::NotMuted => return event,
            MuteState::Permanent => {
                "&cYou are muted and cannot chat.".to_string()
            }
            MuteState::Timed(remaining) => {
                format!(
                    "&cYou are muted for another {}.&r",
                    format_duration(remaining)
                )
            }
        };

        event.cancelled = true;
        event.player.send_system_message(
            TextComponent::from_legacy_string_with_code(&msg, '&'),
            true,
        );
        event
    }
}