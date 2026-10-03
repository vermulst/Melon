//todo:
// /rename, 2
// /more, 2
// /bc/broadcast, 2
// /spawn, 2
// /setspawn, 2
// world, 2
// warp/setwarp/delwarp, 2
// tphere, 2
// tpall, 2
// motd, 2
// top/bottom, 2
// removall <entity>, 2
// playtime, 2
// mute 2
// list 2
// help 2
// getpos 2
// feed 2
// compass 2
// afk 2


// skipped for now
// lastseen
// tpoffline
// repair
// msg/r
// skull

mod command;
mod events;
mod commands;
mod config;

use std::path::PathBuf;
use std::sync::OnceLock;

use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata,
    events::EventPriority,
    permissions::{FS_READ_DATA, FS_WRITE_DATA},
};
use crate::command::register_all_commands;
use crate::events::teleport::TeleportEventListener;

static DATA_FOLDER: OnceLock<PathBuf> = OnceLock::new();

pub fn data_folder() -> Option<PathBuf> {
    DATA_FOLDER.get().cloned()
}

macro_rules! register_events {
    ($context:expr, $priority:expr, $blocking:expr, $( $handler:expr ),* $(,)?) => {
        $(
            $context.register_event_handler($handler, $priority, $blocking)?;
        )*
    };
}

struct Melon;

impl Plugin for Melon {
    fn new() -> Self {
        Melon
    }

    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: "melon".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: vec!["vermulst".into()],
            description: "A plugin for essentials".into(),
            dependencies: vec![],
            permissions: vec![FS_READ_DATA.into(), FS_WRITE_DATA.into()],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        tracing::info!("Melon starting");


        let folder = PathBuf::from(context.get_data_folder());
        let _ = DATA_FOLDER.set(folder.clone());

        config::load(&folder);

        register_events!(
            context,
            EventPriority::Normal,
            true,
            TeleportEventListener,
        );

        register_events!(
            context,
            EventPriority::Normal,
            false,
            crate::events::spawn::SpawnJoinListener,
            crate::events::spawn::RespawnListener,
        );

        register_events!(
            context,
            EventPriority::High,
            true,
            crate::events::mute::MuteFilter,
        );

        register_all_commands(&context);

        tracing::info!("Melon started");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        if let Some(folder) = data_folder() {
            config::save(&folder);
        }
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(Melon);