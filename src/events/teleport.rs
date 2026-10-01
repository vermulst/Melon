use crate::commands::teleport::{BackLocation, TELEPORT_MANAGER};
use pumpkin_plugin_api::{
    events::{EventData, EventHandler, PlayerTeleportEvent},
    Player, Server,
};
use tracing::info;

pub struct TeleportEventListener;

impl EventHandler<PlayerTeleportEvent> for TeleportEventListener {
    fn handle<'a>(
        &'a self,
        server: Server,
        event: EventData<PlayerTeleportEvent>,
    ) -> EventData<PlayerTeleportEvent> {
        let players: Vec<Player> = server.get_all_players();
        if let Some(player) = players
            .iter()
            .find(|p| p.get_id().to_string() == event.player.get_id().to_string())
        {
            let current_pos = player.get_position();
            let current_yaw = player.get_yaw();
            let current_pitch = player.get_pitch();
            let current_world_name = player.get_world().get_name();

            TELEPORT_MANAGER.set_back_location(
                player.get_id().to_string(),
                BackLocation {
                    position: current_pos,
                    yaw: current_yaw,
                    pitch: current_pitch,
                    world_name: current_world_name,
                },
            );

            info!("Saved back location for player {}", player.get_name());
        }

        event
    }
}