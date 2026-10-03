use std::collections::HashSet;

use pumpkin_plugin_api::{
    command::{
        Arg, CommandError, CommandSender, CommandSuggestion, CommandSuggestions,
        ConsumedArgs, SuggestionRequest,
    },
    commands::{CommandHandler, CommandSuggestionHandler},
    text::TextComponent,
    Result, Server,
    wit::pumpkin::plugin::world::EntityType,
};
use crate::commands::utils::CommandSenderExt;

fn display_name(et: EntityType) -> String {
    let s = format!("{et:?}").replace("EntityType::", "");
    let mut out = String::with_capacity(s.len() + 4);
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        for lc in c.to_lowercase() {
            out.push(lc);
        }
    }
    out
}

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn is_hostile(et: EntityType) -> bool {
    matches!(
        et,
        EntityType::Blaze | EntityType::Bogged | EntityType::Breeze
            | EntityType::CaveSpider | EntityType::Creaking | EntityType::Creeper
            | EntityType::Drowned | EntityType::ElderGuardian | EntityType::EnderDragon
            | EntityType::Enderman | EntityType::Endermite | EntityType::Evoker
            | EntityType::Ghast | EntityType::Giant | EntityType::Guardian
            | EntityType::Hoglin | EntityType::Husk | EntityType::Illusioner
            | EntityType::MagmaCube | EntityType::Parched | EntityType::Phantom
            | EntityType::Piglin | EntityType::PiglinBrute | EntityType::Pillager
            | EntityType::Ravager | EntityType::Silverfish | EntityType::Skeleton
            | EntityType::Slime | EntityType::Spider | EntityType::Stray
            | EntityType::Vex | EntityType::Vindicator | EntityType::Warden
            | EntityType::Witch | EntityType::Wither | EntityType::WitherSkeleton
            | EntityType::Zoglin | EntityType::Zombie | EntityType::ZombieVillager
            | EntityType::ZombifiedPiglin
    )
}

fn is_passive(et: EntityType) -> bool {
    matches!(
        et,
        EntityType::Allay | EntityType::Armadillo | EntityType::Axolotl
            | EntityType::Bat | EntityType::Bee | EntityType::Camel
            | EntityType::Cat | EntityType::Chicken | EntityType::Cod
            | EntityType::CopperGolem | EntityType::Cow | EntityType::Dolphin
            | EntityType::Donkey | EntityType::Fox | EntityType::Frog
            | EntityType::GlowSquid | EntityType::Goat | EntityType::HappyGhast
            | EntityType::Horse | EntityType::IronGolem | EntityType::Llama
            | EntityType::Mooshroom | EntityType::Mule | EntityType::Ocelot
            | EntityType::Panda | EntityType::Parrot | EntityType::Pig
            | EntityType::PolarBear | EntityType::Pufferfish | EntityType::Rabbit
            | EntityType::Salmon | EntityType::Sheep | EntityType::Sniffer
            | EntityType::SnowGolem | EntityType::Squid | EntityType::Strider
            | EntityType::Tadpole | EntityType::TraderLlama
            | EntityType::TropicalFish | EntityType::Turtle | EntityType::Villager
            | EntityType::WanderingTrader | EntityType::Wolf
    )
}

fn is_vehicle(et: EntityType) -> bool {
    matches!(
        et,
        EntityType::AcaciaBoat | EntityType::AcaciaChestBoat
            | EntityType::BambooChestRaft | EntityType::BambooRaft
            | EntityType::BirchBoat | EntityType::BirchChestBoat
            | EntityType::CherryBoat | EntityType::CherryChestBoat
            | EntityType::ChestMinecart | EntityType::CommandBlockMinecart
            | EntityType::DarkOakBoat | EntityType::DarkOakChestBoat
            | EntityType::FurnaceMinecart | EntityType::HopperMinecart
            | EntityType::JungleBoat | EntityType::JungleChestBoat
            | EntityType::MangroveBoat | EntityType::MangroveChestBoat
            | EntityType::Minecart | EntityType::OakBoat | EntityType::OakChestBoat
            | EntityType::PaleOakBoat | EntityType::PaleOakChestBoat
            | EntityType::PoplarBoat | EntityType::PoplarChestBoat
            | EntityType::SpawnerMinecart | EntityType::SpruceBoat
            | EntityType::SpruceChestBoat | EntityType::TntMinecart
    )
}

fn is_projectile(et: EntityType) -> bool {
    matches!(
        et,
        EntityType::Arrow | EntityType::BreezeWindCharge | EntityType::DragonFireball
            | EntityType::Egg | EntityType::EnderPearl | EntityType::EvokerFangs
            | EntityType::ExperienceBottle | EntityType::EyeOfEnder
            | EntityType::Fireball | EntityType::FireworkRocket
            | EntityType::FishingBobber | EntityType::LingeringPotion
            | EntityType::LlamaSpit | EntityType::ShulkerBullet
            | EntityType::SmallFireball | EntityType::Snowball
            | EntityType::SpectralArrow | EntityType::SplashPotion
            | EntityType::Trident | EntityType::WindCharge | EntityType::WitherSkull
    )
}

fn matches_filter(et: EntityType, filter: &str) -> bool {
    match filter {
        "monster" | "monsters" | "hostile" | "hostiles" => is_hostile(et),
        "animal" | "animals" | "passive" | "passives" => is_passive(et),
        "mob" | "mobs" | "living" => {
            !matches!(et, EntityType::Player) && !is_vehicle(et) && !is_projectile(et)
        }
        "item" | "items" => matches!(et, EntityType::Item),
        "xp" | "experience" => matches!(et, EntityType::ExperienceOrb),
        "vehicle" | "vehicles" => is_vehicle(et),
        "projectile" | "projectiles" => is_projectile(et),
        _ => normalize(&display_name(et)) == filter,
    }
}

pub struct RemoveAllHandler;

impl CommandHandler for RemoveAllHandler {
    fn handle(
        &self,
        sender: CommandSender,
        _server: Server,
        args: ConsumedArgs,
    ) -> Result<i32, CommandError> {
        let player = sender.require_player("removeall")?;

        let raw = match args.get_value("type") {
            Arg::Simple(s) => s,
            _ => {
                return Err(CommandError::CommandFailed(
                    TextComponent::from_legacy_string_with_code(
                        "&cUsage: /removeall <type|group>",
                        '&',
                    ),
                ));
            }
        };
        let filter = normalize(&raw);

        let world = player.get_world();
        let mut removed = 0usize;

        for entity in world.get_entities() {
            let et = entity.get_type();
            if matches!(et, EntityType::Player) {
                continue;
            }
            if !matches_filter(et, &filter) {
                continue;
            }
            entity.remove();
            removed += 1;
        }

        sender.send_system_message(TextComponent::from_legacy_string_with_code(
            &format!(
                "&aRemoved &e{removed}&a entities matching &e{raw}&a in &e{}&a.&r",
                world.get_name()
            ),
            '&',
        ));

        Ok(0)
    }
}

const GROUPS: &[&str] = &[
    "monster", "hostile", "animal", "passive", "mob",
    "item", "xp", "vehicle", "projectile",
];

pub struct RemoveAllSuggestions;

impl CommandSuggestionHandler for RemoveAllSuggestions {
    fn suggest(
        &self,
        sender: CommandSender,
        _server: Server,
        request: SuggestionRequest,
    ) -> CommandSuggestions {
        let prefix: &str = &request.remaining;
        let prefix_lower = prefix.to_lowercase();

        let mut seen: HashSet<String> = HashSet::new();
        let mut values: Vec<CommandSuggestion> = Vec::new();

        for g in GROUPS {
            if g.starts_with(&prefix_lower) && seen.insert((*g).to_string()) {
                values.push(CommandSuggestion {
                    value: (*g).to_string(),
                    tooltip: None,
                });
            }
        }

        if let Some(player) = sender.as_player() {
            let world = player.get_world();
            for entity in world.get_entities() {
                let et = entity.get_type();
                if matches!(et, EntityType::Player) {
                    continue;
                }
                let name = display_name(et);
                if name.starts_with(&prefix_lower) && seen.insert(name.clone()) {
                    values.push(CommandSuggestion {
                        value: name,
                        tooltip: None,
                    });
                }
            }
        }

        values.sort_by(|a, b| a.value.cmp(&b.value));

        CommandSuggestions {
            start: request.start,
            length: prefix.len() as u32,
            values,
        }
    }
}