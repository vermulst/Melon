use pumpkin_plugin_api::{
    command::{CommandError, CommandSender, ConsumedArgs},
    commands::CommandHandler,
    text::TextComponent,
    Result, Server,
};

const COMMANDS: &[(&str, &str)] = &[
    ("afk",        "Toggle your AFK status"),
    ("back",       "Return to your previous location"),
    ("bottom",     "Teleport to the lowest safe surface"),
    ("broadcast",  "Broadcast a message to all players"),
    ("compass",    "Show your facing direction"),
    ("delwarp",    "Delete a warp"),
    ("feed",       "Restore hunger"),
    ("fly",        "Toggle flight mode"),
    ("flyspeed",   "Change flight speed"),
    ("gamemode",   "Change gamemode by name or ID"),
    ("getpos",     "Show your coordinates"),
    ("gma",        "Switch gamemode to Adventure"),
    ("gmc",        "Switch gamemode to Creative"),
    ("gms",        "Switch gamemode to Survival"),
    ("gmsp",       "Switch gamemode to Spectator"),
    ("heal",       "Restore health and hunger"),
    ("help",       "Show this help"),
    ("invsee",     "View another player's inventory"),
    ("list",       "List online players"),
    ("melonreload","Reload Melon config"),
    ("more",       "Fill item in hand to max stack"),
    ("motd",       "Show the message of the day"),
    ("mute",       "Mute or unmute a player"),
    ("ping",       "Show latency"),
    ("playtime",   "Show playtime"),
    ("removeall",  "Remove entities in bulk"),
    ("rename",     "Rename the item in your hand"),
    ("setspawn",   "Set the world spawn"),
    ("setwarp",    "Create a warp"),
    ("spawn",      "Teleport to spawn"),
    ("top",        "Teleport to the highest safe surface"),
    ("tpall",      "Teleport everyone to you"),
    ("tphere",     "Teleport a player to you"),
    ("warp",       "Teleport to a warp, or list warps"),
    ("world",      "Teleport to another world's spawn"),
];

pub struct HelpHandler;

impl CommandHandler for HelpHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        _args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!("&6Melon commands (&e{}&6):", COMMANDS.len()),
            '&',
        ));

        for (name, desc) in COMMANDS {
            sender.send_system_message(TextComponent::from_legacy_string_with_code(
                &format!("&e/{name} &7- &f{desc}"),
                '&',
            ));
        }

        Ok(0)
    }
}