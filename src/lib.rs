//todo: /repair, /rename, /more, /bc/broadcast, /spawn, /setspawn

mod command;
mod events;
mod commands;

use pumpkin_plugin_api::command::Command;
use pumpkin_plugin_api::player::PermissionLevel;
use pumpkin_plugin_api::{
    Context, Plugin, PluginMetadata, Server,
    events::EventPriority,
    permission::{Permission, PermissionDefault},
};
use crate::command::register_all_commands;
use crate::events::teleport::TeleportEventListener;

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
            permissions: vec![],
        }
    }

    fn on_load(&self, context: Context) -> pumpkin_plugin_api::Result<()> {
        tracing::info!("Melon starting");

        register_events!(
            context,
            EventPriority::Normal,
            true,
            TeleportEventListener,
        );

        register_all_commands(&context);


        tracing::info!("Melon started");
        Ok(())
    }

    fn on_unload(&self, _context: Context) -> pumpkin_plugin_api::Result<()> {
        Ok(())
    }
}

pumpkin_plugin_api::register_plugin!(Melon);