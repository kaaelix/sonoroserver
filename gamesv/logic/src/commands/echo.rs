use std::io;

use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::phantom_item::PhantomItem as PhantomItemConfig;
use sonoro_gamesv_data::tables::phantom_main_property::PhantomMainProperty;
use sonoro_gamesv_data::tables::phantom_sub_property::PhantomSubProperty;
use sonoro_gamesv_fs::echo_info::{self, EchoInfo, Prop};
use sonoro_gamesv_fs::special_item_incr;
use sonoro_proto::pb::PhantomItemAddNotify;

use crate::commands::respond;
use crate::component::comp_util;
use crate::component::player::PlayerEchoComponent;
use crate::event_queue::EventQueue;
use crate::sink::PushSink;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct MainStatDef {
    letter: u8,
    prop_id: i32,
    add_type: i32,
    name: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct SubStatDef {
    letter: u8,
    prop_id: i32,
    add_type: i32,
    name: &'static str,
}

struct SubStatInput<'a> {
    letter: Option<&'a str>,
    tier: Option<i32>,
}

const MAIN_STATS: &[MainStatDef] = &[
    MainStatDef {
        letter: b'a',
        prop_id: 8,
        add_type: 1,
        name: "Crit Rate",
    },
    MainStatDef {
        letter: b'b',
        prop_id: 9,
        add_type: 1,
        name: "Crit DMG",
    },
    MainStatDef {
        letter: b'c',
        prop_id: 10007,
        add_type: 2,
        name: "ATK%",
    },
    MainStatDef {
        letter: b'd',
        prop_id: 10002,
        add_type: 2,
        name: "HP%",
    },
    MainStatDef {
        letter: b'e',
        prop_id: 10010,
        add_type: 2,
        name: "DEF%",
    },
    MainStatDef {
        letter: b'f',
        prop_id: 35,
        add_type: 1,
        name: "Healing Bonus",
    },
    MainStatDef {
        letter: b'g',
        prop_id: 22,
        add_type: 1,
        name: "Glacio DMG Bonus",
    },
    MainStatDef {
        letter: b'h',
        prop_id: 23,
        add_type: 1,
        name: "Fusion DMG Bonus",
    },
    MainStatDef {
        letter: b'i',
        prop_id: 24,
        add_type: 1,
        name: "Electro DMG Bonus",
    },
    MainStatDef {
        letter: b'j',
        prop_id: 25,
        add_type: 1,
        name: "Aero DMG Bonus",
    },
    MainStatDef {
        letter: b'k',
        prop_id: 26,
        add_type: 1,
        name: "Spectro DMG Bonus",
    },
    MainStatDef {
        letter: b'l',
        prop_id: 27,
        add_type: 1,
        name: "Havoc DMG Bonus",
    },
    MainStatDef {
        letter: b'm',
        prop_id: 11,
        add_type: 1,
        name: "Energy Regen",
    },
];

const SUB_STATS: &[SubStatDef] = &[
    SubStatDef {
        letter: b'a',
        prop_id: 10002,
        add_type: 1,
        name: "Flat HP",
    },
    SubStatDef {
        letter: b'b',
        prop_id: 10007,
        add_type: 1,
        name: "Flat ATK",
    },
    SubStatDef {
        letter: b'c',
        prop_id: 10010,
        add_type: 1,
        name: "Flat DEF",
    },
    SubStatDef {
        letter: b'd',
        prop_id: 10002,
        add_type: 2,
        name: "HP%",
    },
    SubStatDef {
        letter: b'e',
        prop_id: 10007,
        add_type: 2,
        name: "ATK%",
    },
    SubStatDef {
        letter: b'f',
        prop_id: 10010,
        add_type: 2,
        name: "DEF%",
    },
    SubStatDef {
        letter: b'g',
        prop_id: 14,
        add_type: 1,
        name: "Resonance Skill DMG Bonus",
    },
    SubStatDef {
        letter: b'h',
        prop_id: 17,
        add_type: 1,
        name: "Basic Attack DMG Bonus",
    },
    SubStatDef {
        letter: b'i',
        prop_id: 18,
        add_type: 1,
        name: "Heavy Attack DMG Bonus",
    },
    SubStatDef {
        letter: b'j',
        prop_id: 19,
        add_type: 1,
        name: "Resonance Liberation DMG Bonus",
    },
    SubStatDef {
        letter: b'k',
        prop_id: 8,
        add_type: 1,
        name: "Crit Rate",
    },
    SubStatDef {
        letter: b'l',
        prop_id: 9,
        add_type: 1,
        name: "Crit DMG",
    },
    SubStatDef {
        letter: b'm',
        prop_id: 11,
        add_type: 1,
        name: "Energy Regen",
    },
];

#[allow(clippy::module_inception)]
pub mod echo {
    use super::*;

    pub const ALIAS: &str = "ec";
    pub const DESCRIPTION: &str = "creates a custom Echo.\nusage: echo [echo_id] [sonata_id] [main] [sub] [tier] ...\n\nMain stat a-m:\na CR, b CD, c ATK%, d HP%, e DEF%, f Heal\ng Glacio, h Fusion, i Electro, j Aero\nk Spectro, l Havoc, m Energy\n\nSub stats a-m:\na HP, b ATK, c DEF, d HP%, e ATK%, f DEF%\ng Skill, h Basic, i Heavy, j Liberation\nk CR, l CD, m Energy\n\ntier: stat roll value. 1 is the lowest, higher is better.";

    #[allow(clippy::too_many_arguments)]
    pub fn call<S: PushSink + ?Sized>(
        events: &mut EventQueue,
        assets: &Assets,
        fs: &FileSystem,
        sink: &mut S,
        echo_comp: &mut PlayerEchoComponent,
        echo_item_id: i32,
        sonata_id: i32,
        main_letter: &str,
        sub1: Option<&str>,
        tier1: Option<i32>,
        sub2: Option<&str>,
        tier2: Option<i32>,
        sub3: Option<&str>,
        tier3: Option<i32>,
        sub4: Option<&str>,
        tier4: Option<i32>,
        sub5: Option<&str>,
        tier5: Option<i32>,
    ) -> io::Result<()> {
        let player_id = echo_comp.player_id;
        let Some(item) = assets.tables.phantom_item.get_data_by_id(&echo_item_id) else {
            respond(events, player_id, format!("unknown Echo item id: {echo_item_id}"));
            return Ok(());
        };
        if !echo_info::valid_default_owned_item(assets, &item) {
            respond(
                events,
                player_id,
                format!("Echo item {echo_item_id} cannot be owned"),
            );
            return Ok(());
        }

        let Some(fetter_group_id) = resolve_fetter_group(events, player_id, &item, sonata_id) else {
            return Ok(());
        };
        let Some(main_prop) = resolve_main_property(
            events,
            player_id,
            assets,
            item.MainProp.RandGroupId,
            main_letter,
        ) else {
            return Ok(());
        };

        let sub_inputs = [
            SubStatInput {
                letter: sub1,
                tier: tier1,
            },
            SubStatInput {
                letter: sub2,
                tier: tier2,
            },
            SubStatInput {
                letter: sub3,
                tier: tier3,
            },
            SubStatInput {
                letter: sub4,
                tier: tier4,
            },
            SubStatInput {
                letter: sub5,
                tier: tier5,
            },
        ];

        let Some(sub_props) =
            build_sub_props(events, player_id, assets, item.QualityId, &sub_inputs)
        else {
            return Ok(());
        };

        let Some(level) = max_level_for_quality(events, player_id, assets, item.QualityId) else {
            return Ok(());
        };
        let main_props =
            echo_info::build_main_props_from_group(assets, &main_prop.PropGroup, level);

        let incr_id = special_item_incr::next(fs, player_id)?;
        let new_echo = EchoInfo {
            id: echo_item_id,
            level,
            main_prop: main_props,
            sub_prop: sub_props,
            fetter_group_id,
            ..Default::default()
        };

        echo_comp.echo_map.insert(incr_id, new_echo);
        if let Err(err) = save_echo_info(fs, player_id, incr_id, echo_comp.echo_map.get(&incr_id).expect("just inserted")) {
            echo_comp.echo_map.remove(&incr_id);
            return Err(err);
        }

        let pb_item = echo_comp
            .echo_map
            .get(&incr_id)
            .expect("inserted")
            .to_proto(incr_id);
        sink.push(
            PhantomItemAddNotify::MSG_ID,
            &PhantomItemAddNotify {
                PhantomItemList: vec![pb_item],
                Reason: 0,
            },
        );

        respond(
            events,
            player_id,
            format!("created Echo {echo_item_id} with incr id {incr_id}"),
        );
        Ok(())
    }
}

fn resolve_fetter_group(
    events: &mut EventQueue,
    player_id: i32,
    item: &PhantomItemConfig,
    sonata_id: i32,
) -> Option<i32> {
    if item.FetterGroup.contains(&sonata_id) {
        return Some(sonata_id);
    }
    let valid: String = item
        .FetterGroup
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    respond(
        events,
        player_id,
        format!("Sonata {sonata_id} is invalid for this Echo; valid: {valid}"),
    );
    None
}

fn build_sub_props(
    events: &mut EventQueue,
    player_id: i32,
    assets: &Assets,
    quality: i32,
    inputs: &[SubStatInput<'_>],
) -> Option<Vec<Prop>> {
    let Some(max_count) = echo_info::max_unlocked_sub_prop_count(assets, quality) else {
        respond(events, player_id, format!("unknown Echo quality: {quality}"));
        return None;
    };

    let mut props = Vec::new();
    for input in inputs {
        if input.letter.is_none() && input.tier.is_none() {
            continue;
        }
        let (Some(letter), Some(tier)) = (input.letter, input.tier) else {
            respond(
                events,
                player_id,
                "substat arguments must be letter/tier pairs",
            );
            return None;
        };
        if props.len() >= max_count {
            respond(
                events,
                player_id,
                format!("Echo quality {quality} allows only {max_count} substats"),
            );
            return None;
        }

        let Some(def) = resolve_sub_stat_def(letter) else {
            respond(events, player_id, format!("invalid substat letter: {letter}"));
            return None;
        };
        let Some(prop) = find_sub_property(assets, def) else {
            respond(
                events,
                player_id,
                format!("missing substat data for {}", def.name),
            );
            return None;
        };
        if echo_info::contains_sub_prop_role(assets, &props, prop) {
            respond(events, player_id, format!("duplicate substat: {}", def.name));
            return None;
        }

        let Some(value) = echo_info::sub_prop_value_for_tier(quality, prop, tier) else {
            respond(
                events,
                player_id,
                format!(
                    "{} tier must be 1-{} for quality {quality}",
                    def.name,
                    echo_info::max_sub_prop_tier(quality, prop)
                ),
            );
            return None;
        };
        props.push(Prop { id: prop.Id, value });
    }

    Some(props)
}

fn max_level_for_quality(
    events: &mut EventQueue,
    player_id: i32,
    assets: &Assets,
    quality: i32,
) -> Option<i32> {
    let Some(config) = assets.tables.phantom_quality.get_data_by_id(&quality) else {
        respond(events, player_id, format!("unknown Echo quality: {quality}"));
        return None;
    };
    Some(config.LevelLimit)
}

fn resolve_main_property<'a>(
    events: &mut EventQueue,
    player_id: i32,
    assets: &'a Assets,
    rand_group_id: i32,
    token: &str,
) -> Option<&'a PhantomMainProperty> {
    let Some(letter) = parse_letter(token) else {
        respond(events, player_id, "main stat must be a single letter");
        return None;
    };
    let Some(def) = resolve_main_stat_def(letter) else {
        respond(events, player_id, format!("invalid main stat letter: {token}"));
        return None;
    };

    for group in &assets.tables.phantom_main_property.items {
        if group.RandGroupId != rand_group_id || group.PropGroup.is_empty() {
            continue;
        }
        let Some(prop_item) = assets
            .tables
            .phantom_main_prop_item
            .get_data_by_id(&group.PropGroup[0])
        else {
            continue;
        };
        if prop_item.PropId == def.prop_id && prop_item.AddType == def.add_type {
            return Some(group);
        }
    }

    let valid = build_valid_main_letters(assets, rand_group_id);
    respond(
        events,
        player_id,
        format!("main stat {token} is invalid for this Echo; valid: {valid}"),
    );
    None
}

fn build_valid_main_letters(assets: &Assets, rand_group_id: i32) -> String {
    let mut valid = String::new();
    for def in MAIN_STATS {
        let matches = assets.tables.phantom_main_property.items.iter().any(|group| {
            group.RandGroupId == rand_group_id
                && !group.PropGroup.is_empty()
                && assets
                    .tables
                    .phantom_main_prop_item
                    .get_data_by_id(&group.PropGroup[0])
                    .is_some_and(|p| p.PropId == def.prop_id && p.AddType == def.add_type)
        });
        if matches {
            if !valid.is_empty() {
                valid.push(' ');
            }
            valid.push(def.letter as char);
        }
    }
    valid
}

fn resolve_main_stat_def(letter: u8) -> Option<MainStatDef> {
    MAIN_STATS.iter().copied().find(|def| def.letter == letter)
}

fn resolve_sub_stat_def(token: &str) -> Option<SubStatDef> {
    let letter = parse_letter(token)?;
    SUB_STATS.iter().copied().find(|def| def.letter == letter)
}

fn find_sub_property(assets: &Assets, def: SubStatDef) -> Option<&PhantomSubProperty> {
    assets
        .tables
        .phantom_sub_property
        .items
        .iter()
        .find(|prop| prop.PropId == def.prop_id && prop.AddType == def.add_type)
}

fn parse_letter(token: &str) -> Option<u8> {
    let bytes = token.as_bytes();
    if bytes.len() != 1 {
        return None;
    }
    let letter = bytes[0].to_ascii_lowercase();
    letter.is_ascii_lowercase().then_some(letter)
}

fn save_echo_info(fs: &FileSystem, player_id: i32, incr_id: i32, item: &EchoInfo) -> io::Result<()> {
    let path = format!("player/{player_id}/{}/{incr_id}", EchoInfo::DATA_DIR);
    comp_util::save_struct(fs, item, &path)
}
