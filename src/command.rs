use pumpkin_plugin_api::Context;
use pumpkin_plugin_api::command::{ArgumentType, Command, CommandNode, StringType};
use pumpkin_plugin_api::commands::CommandHandler;
use pumpkin_plugin_api::context::Permission;
use pumpkin_plugin_api::permission::PermissionDefault;
use pumpkin_plugin_api::player::PermissionLevel;
use crate::commands::{BackHandler, DynamicGamemodeHandler, FixedGamemodeHandler, FlyHandler, FlySpeedHandler, HealHandler, InvseeHandler, Mode, PingHandler, RenameHandler};

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
                names: &["flyspeed"],
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
    ]
}