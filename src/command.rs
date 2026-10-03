use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::{ArgumentType, Command, CommandNode, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::context::Permission;
use pumpkin_plugin_api::permission::PermissionDefault;
use pumpkin_plugin_api::player::PermissionLevel;
use crate::commands::{AfkHandler, BackHandler, BottomHandler, BroadcastHandler, CompassHandler, DelWarpHandler, DynamicGamemodeHandler, FeedHandler, FixedGamemodeHandler, FlyHandler, FlySpeedHandler, GetPosHandler, HealHandler, HelpHandler, InvseeHandler, ListHandler, Mode, MoreHandler, MotdHandler, MuteHandler, PingHandler, PlaytimeHandler, ReloadHandler, RemoveAllHandler, RemoveAllSuggestions, RenameHandler, SetSpawnHandler, SetWarpHandler, SpawnHandler, TopHandler, TpAllHandler, TpHereHandler, WarpHandler, WarpNameSuggestions, WorldHandler};
use crate::commands::mute::DurationSuggestions;

pub struct CommandRegistration<F> {
    pub names: &'static [&'static str],
    pub description: &'static str,
    pub permission: &'static str,
    pub perm_description: &'static str,
    pub level: PermissionLevel,
    pub nodes: Vec<CommandNode>,
    pub handler_builder: F,
}

impl<H: CommandHandler + 'static, F: Fn() -> H> CommandRegistration<F> {
    pub fn register(self, context: &Context) {
        context.register_permission(&Permission {
            node: self.permission.to_string(),
            description: self.perm_description.to_string(),
            default: PermissionDefault::Op(self.level),
            children: Vec::new(),
        });

        let names: Vec<String> = self.names.iter().map(|&s| s.to_string()).collect();
        let mut command = Command::new(&names, self.description);

        for node in self.nodes {
            command = command.then(node);
        }

        let command = command.execute((self.handler_builder)());
        context.register_command(command, self.permission);
    }
}

pub fn register_all_commands(context: &Context) {
    for register_fn in commands() {
        register_fn(context);
    }
}

pub fn commands() -> Vec<fn(&Context)> {
    vec![
        |ctx| {
            CommandRegistration {
                names: &["invsee"],
                description: "View another player's inventory",
                permission: "melon:invsee.use",
                perm_description: "Allows viewing another player's inventory",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(InvseeHandler),
                ],
                handler_builder: || InvseeHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["ping"],
                description: "Check your ping or another player's ping",
                permission: "melon:ping.use",
                perm_description: "Allows checking latency",
                level: PermissionLevel::Zero,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(PingHandler),
                ],
                handler_builder: || PingHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["fly"],
                description: "Toggle flight mode for yourself or another player",
                permission: "melon:fly.use",
                perm_description: "Allows toggling flight mode",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FlyHandler),
                ],
                handler_builder: || FlyHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["flyspeed", "fspeed"],
                description: "Set flight speed for yourself or another player (1-20)",
                permission: "melon:flyspeed.use",
                perm_description: "Allows changing flight speed",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument(
                        "speed",
                        &ArgumentType::Float((Some(1.0), Some(20.0))),
                    )
                        .execute(FlySpeedHandler)
                        .then(
                            CommandNode::argument("target", &ArgumentType::Players)
                                .execute(FlySpeedHandler),
                        ),
                ],
                handler_builder: || FlySpeedHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["heal"],
                description: "Heal yourself or another player",
                permission: "melon:heal.use",
                perm_description: "Allows healing players",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(HealHandler),
                ],
                handler_builder: || HealHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["gamemode", "gm"],
                description: "Change gamemode using mode name or numerical ID (0-3)",
                permission: "melon:gamemode.use",
                perm_description: "Allows changing gamemode by ID or name",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("mode", &ArgumentType::String(StringType::SingleWord))
                        .execute(DynamicGamemodeHandler)
                        .then(
                            CommandNode::argument("target", &ArgumentType::Players)
                                .execute(DynamicGamemodeHandler),
                        ),
                ],
                handler_builder: || DynamicGamemodeHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["gms"],
                description: "Switch gamemode to Survival",
                permission: "melon:gamemode.survival",
                perm_description: "Allows switching to survival mode",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FixedGamemodeHandler { mode: Mode::Survival }),
                ],
                handler_builder: || FixedGamemodeHandler { mode: Mode::Survival },
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["gmc"],
                description: "Switch gamemode to Creative",
                permission: "melon:gamemode.creative",
                perm_description: "Allows switching to creative mode",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FixedGamemodeHandler { mode: Mode::Creative }),
                ],
                handler_builder: || FixedGamemodeHandler { mode: Mode::Creative },
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["gma"],
                description: "Switch gamemode to Adventure",
                permission: "melon:gamemode.adventure",
                perm_description: "Allows switching to adventure mode",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FixedGamemodeHandler { mode: Mode::Adventure }),
                ],
                handler_builder: || FixedGamemodeHandler { mode: Mode::Adventure },
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["gmsp"],
                description: "Switch gamemode to Spectator",
                permission: "melon:gamemode.spectator",
                perm_description: "Allows switching to spectator mode",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FixedGamemodeHandler { mode: Mode::Spectator }),
                ],
                handler_builder: || FixedGamemodeHandler { mode: Mode::Spectator },
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["back"],
                description: "Teleport back to your previous location",
                permission: "melon:back.use",
                perm_description: "Allows returning to previous location",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || BackHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["rename"],
                description: "Rename the item in your main hand",
                permission: "melon:rename.use",
                perm_description: "Allows renaming items",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("name", &ArgumentType::String(StringType::Greedy))
                        .execute(RenameHandler),
                ],
                handler_builder: || RenameHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["more"],
                description: "Gives you the max stack amount of the item in your main hand",
                permission: "melon:more.use",
                perm_description: "Allows duping items",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || MoreHandler,
            }
                .register(ctx);

        },
        |ctx| {
            CommandRegistration {
                names: &["broadcast", "bc"],
                description: "Broadcast a message to all players",
                permission: "melon:broadcast.use",
                perm_description: "Allows broadcasting server-wide messages",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("message", &ArgumentType::String(StringType::Greedy))
                        .execute(BroadcastHandler),
                ],
                handler_builder: || BroadcastHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["feed"],
                description: "Restore your food level or another player's food level",
                permission: "melon:feed.use",
                perm_description: "Allows feeding players",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(FeedHandler),
                ],
                handler_builder: || FeedHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["compass"],
                description: "Display your current facing direction",
                permission: "melon:compass.use",
                perm_description: "Allows checking your cardinal direction",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || CompassHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["getpos", "coords", "pos"],
                description: "Display your current coordinates and orientation",
                permission: "melon:getpos.use",
                perm_description: "Allows checking your current position",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || GetPosHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["top"],
                description: "Teleport to the highest position at your location",
                permission: "melon:top.use",
                perm_description: "Allows teleporting to the top",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || TopHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["bottom"],
                description: "Teleport to the lowest position at your location",
                permission: "melon:bottom.use",
                perm_description: "Allows teleporting to the bottom",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || BottomHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["spawn"],
                description: "Teleport to the server spawn point",
                permission: "melon:spawn.use",
                perm_description: "Allows teleporting to spawn",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || SpawnHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["setspawn"],
                description: "Set the world spawn to your current location",
                permission: "melon:setspawn.use",
                perm_description: "Allows setting the world spawn",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || SetSpawnHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["playtime", "pt", "ptime"],
                description: "Check how long you or another player have played",
                permission: "melon:playtime.use",
                perm_description: "Allows checking playtime",
                level: PermissionLevel::Zero,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(PlaytimeHandler),
                ],
                handler_builder: || PlaytimeHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["melonreload", "mreload"],
                description: "Reload the Melon plugin config from disk",
                permission: "melon:reload.use",
                perm_description: "Allows reloading the Melon config",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || ReloadHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["mute"],
                description: "Toggle a player's chat mute, optionally for a duration",
                permission: "melon:mute.use",
                perm_description: "Allows muting and unmuting players",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(MuteHandler)
                        .then(
                            CommandNode::argument(
                                "duration",
                                &ArgumentType::String(StringType::SingleWord),
                            )
                                .suggest(DurationSuggestions)
                                .execute(MuteHandler),
                        ),
                ],
                handler_builder: || MuteHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["tphere"],
                description: "Teleport a player to you",
                permission: "melon:tphere.use",
                perm_description: "Allows teleporting other players to yourself",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("target", &ArgumentType::Players)
                        .execute(TpHereHandler),
                ],
                handler_builder: || TpHereHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["tpall"],
                description: "Teleport every player to you",
                permission: "melon:tpall.use",
                perm_description: "Allows teleporting every player to yourself",
                level: PermissionLevel::Two,
                nodes: vec![],
                handler_builder: || TpAllHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["motd"],
                description: "Show the message of the day",
                permission: "melon:motd.use",
                perm_description: "Allows viewing the server MOTD",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || MotdHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["setwarp"],
                description: "Create a warp at your location",
                permission: "melon:setwarp.use",
                perm_description: "Allows creating warps",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .suggest(WarpNameSuggestions)
                        .execute(SetWarpHandler),
                ],
                handler_builder: || SetWarpHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["delwarp"],
                description: "Delete a warp",
                permission: "melon:delwarp.use",
                perm_description: "Allows deleting warps",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .suggest(WarpNameSuggestions)
                        .execute(DelWarpHandler),
                ],
                handler_builder: || DelWarpHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["warp"],
                description: "Teleport to a warp, or list warps",
                permission: "melon:warp.use",
                perm_description: "Allows using warps",
                level: PermissionLevel::Zero,
                nodes: vec![
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .suggest(WarpNameSuggestions)
                        .execute(WarpHandler),
                ],
                handler_builder: || WarpHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["removeall", "killall"],
                description: "Remove all entities of a given type in your current world",
                permission: "melon:removeall.use",
                perm_description: "Allows removing entities in bulk",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("type", &ArgumentType::String(StringType::SingleWord))
                        .suggest(RemoveAllSuggestions)
                        .execute(RemoveAllHandler),
                ],
                handler_builder: || RemoveAllHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["list", "online", "who"],
                description: "List online players",
                permission: "melon:list.use",
                perm_description: "Allows listing online players",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || ListHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["afk", "away"],
                description: "Toggle your AFK status",
                permission: "melon:afk.use",
                perm_description: "Allows toggling AFK",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || AfkHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["help"],
                description: "Show available commands",
                permission: "melon:help.use",
                perm_description: "Allows viewing help",
                level: PermissionLevel::Zero,
                nodes: vec![],
                handler_builder: || HelpHandler,
            }
                .register(ctx);
        },
        |ctx| {
            CommandRegistration {
                names: &["world"],
                description: "Teleport to another world's spawn",
                permission: "melon:world.use",
                perm_description: "Allows switching worlds",
                level: PermissionLevel::Two,
                nodes: vec![
                    CommandNode::argument("name", &ArgumentType::String(StringType::SingleWord))
                        .execute(WorldHandler),
                ],
                handler_builder: || WorldHandler,
            }
                .register(ctx);
        },
    ]
}