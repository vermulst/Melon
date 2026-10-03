use std::sync::{LazyLock, Mutex};

use pumpkin_plugin_api::{
    events::{EventData, EventHandler, PlayerJoinEvent, PlayerRespawnEvent},
    player::CustomStatistic,
    text::TextComponent,
    Server,
};
use pumpkin_plugin_api::scheduler::SchedulerExt;
use crate::config::CONFIG;

const FIRST_JOIN_TICK_THRESHOLD: u64 = 20;

static PENDING_FIRST_JOIN: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static PENDING_RESPAWN: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub struct SpawnJoinListener;

impl EventHandler<PlayerJoinEvent> for SpawnJoinListener {
    fn handle(
        &self,
        server: Server,
        event: EventData<PlayerJoinEvent>,
    ) -> EventData<PlayerJoinEvent> {
        let (first_join_cfg, every_join_cfg) = {
            let cfg = CONFIG.lock().unwrap();
            (
                cfg.spawn.force_spawn_on_first_join,
                cfg.spawn.force_spawn_on_every_join,
            )
        };

        let ticks =
            event.player.get_custom_statistic(CustomStatistic::TotalWorldTime) as u64;
        let is_first_join = ticks < FIRST_JOIN_TICK_THRESHOLD;

        let should_teleport = every_join_cfg || (first_join_cfg && is_first_join);
        if !should_teleport {
            return event;
        }

        PENDING_FIRST_JOIN
            .lock()
            .unwrap()
            .push(event.player.get_id().to_string());

        server.schedule_delayed_task(2, run_pending_spawns);

        event
    }
}

pub struct RespawnListener;

impl EventHandler<PlayerRespawnEvent> for RespawnListener {
    fn handle(
        &self,
        server: Server,
        event: EventData<PlayerRespawnEvent>,
    ) -> EventData<PlayerRespawnEvent> {
        let should_teleport = CONFIG
            .lock()
            .unwrap()
            .spawn
            .force_spawn_on_respawn;

        if !should_teleport {
            return event;
        }

        PENDING_RESPAWN
            .lock()
            .unwrap()
            .push(event.player.get_id().to_string());

        server.schedule_delayed_task(2, run_pending_respawns);

        event
    }
}

fn teleport_to_spawn(player: &mut pumpkin_plugin_api::Player) {
    let world = player.get_world();
    let world_name = world.get_name();

    let custom = CONFIG
        .lock()
        .unwrap()
        .spawn_points
        .get(&world_name)
        .copied();

    if let Some(sp) = custom {
        player.teleport(
            (sp.x, sp.y, sp.z),
            Some(sp.yaw),
            Some(sp.pitch),
            world,
        );
    } else {
        let loc = world.get_spawn_location();
        let p = loc.pos;
        player.teleport(
            (p.x as f64, p.y as f64, p.z as f64),
            Some(loc.yaw),
            Some(loc.pitch),
            world,
        );
    }
}

fn run_pending_spawns(server: Server) {
    let pending: Vec<String> = PENDING_FIRST_JOIN.lock().unwrap().drain(..).collect();
    if pending.is_empty() {
        return;
    }

    for mut player in server.get_all_players() {
        let uuid = player.get_id().to_string();
        if !pending.iter().any(|p| p == &uuid) {
            continue;
        }

        teleport_to_spawn(&mut player);

        player.send_system_message(
            TextComponent::from_legacy_string_with_code(
                "&aWelcome! Teleporting to spawn.&r",
                '&',
            ),
            false,
        );
    }
}

fn run_pending_respawns(server: Server) {
    let pending: Vec<String> = PENDING_RESPAWN.lock().unwrap().drain(..).collect();
    if pending.is_empty() {
        return;
    }

    for mut player in server.get_all_players() {
        let uuid = player.get_id().to_string();
        if !pending.iter().any(|p| p == &uuid) {
            continue;
        }

        teleport_to_spawn(&mut player);
    }
}