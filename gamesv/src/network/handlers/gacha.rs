use std::cmp::Ordering;

use sonoro_gamesv_data::Assets;
use sonoro_gamesv_data::tables::gacha_view_info::GachaViewInfo;
use sonoro_gamesv_data::zon_config::GameConfig;
use sonoro_gamesv_fs::gacha_info::{Banner, Bucket, GachaInfo};
use sonoro_gamesv_fs::inventory_info::InventoryInfo;
use sonoro_gamesv_fs::role_info::RoleInfo;
use sonoro_gamesv_fs::special_item_incr;
use sonoro_gamesv_fs::weapon_item::WeaponItem;
use sonoro_gamesv_logic::component::comp_util;
use sonoro_gamesv_logic::component::player::{
    PlayerGachaComponent, PlayerInventoryComponent, PlayerWeaponComponent,
};
use sonoro_gamesv_logic::helpers::consume::CostItem;
use sonoro_gamesv_logic::helpers::{consume, inventory as inventory_helper, role as role_helper, role_stats};
use sonoro_proto::enums::ErrorCode;
use sonoro_proto::pb::{
    GachaConsume, GachaInfo as PbGachaInfo, GachaInfoRequest, GachaInfoResponse, GachaItem,
    GachaPoolDetail, GachaPoolDetailRequest, GachaPoolDetailResponse, GachaPoolInfo, GachaRequest,
    GachaResponse, GachaResult, GachaReward, GachaUsePoolRequest, GachaUsePoolResponse, NormalItem,
    NormalItemAddNotify, NormalItemUpdateNotify, PbGetRoleListNotify, WeaponItem as PbWeaponItem,
    WeaponItemAddNotify,
};

use super::State;

mod view_type {
    pub const NOVICE: i32 = 1;
    pub const FEATURED_ROLE: i32 = 2;
    pub const FEATURED_WEAPON: i32 = 3;
    pub const STANDARD_ROLE: i32 = 4;
    pub const STANDARD_WEAPON: i32 = 5;
    pub const BEGINNER_CHOICE: i32 = 6;
    pub const ANNIVERSARY_ROLE: i32 = 7;
    pub const ANNIVERSARY_WEAPON: i32 = 8;
    pub const NEW_VOYAGE_ROLE: i32 = 9;
    pub const NEW_VOYAGE_WEAPON: i32 = 10;
    pub const COLLAB_ROLE: i32 = 11;
    pub const COLLAB_WEAPON: i32 = 12;
    pub const RERUN_ROLE: i32 = 13;
    pub const RERUN_WEAPON: i32 = 14;
}

const ITEM_ID_AFTERGLOW_CORAL: i32 = 50003;
const ITEM_ID_OSCILLATED_CORAL: i32 = 50004;

fn save_inventory_info(
    fs: &sonoro_common::fs::FileSystem,
    inventory_comp: &PlayerInventoryComponent,
) -> Result<(), String> {
    let path = format!(
        "player/{}/{}",
        inventory_comp.player_id,
        InventoryInfo::DATA_PATH
    );
    comp_util::save_struct(fs, &inventory_comp.info, &path).map_err(|e| e.to_string())
}

fn save_weapon_info(
    fs: &sonoro_common::fs::FileSystem,
    weapon_comp: &PlayerWeaponComponent,
    incr_id: i32,
    weapon: &WeaponItem,
) -> Result<(), String> {
    let path = format!(
        "player/{}/{}/{}",
        weapon_comp.player_id,
        WeaponItem::DATA_DIR,
        incr_id
    );
    comp_util::save_struct(fs, weapon, &path).map_err(|e| e.to_string())
}

fn save_gacha_info(
    fs: &sonoro_common::fs::FileSystem,
    comp: &PlayerGachaComponent,
) -> Result<(), String> {
    let path = format!("player/{}/{}", comp.player_id, GachaInfo::DATA_PATH);
    comp_util::save_struct(fs, &comp.info, &path).map_err(|e| e.to_string())
}

fn role_waveband_item(assets: &Assets, role_id: i32) -> Option<i32> {
    let role = assets.tables.role_info.get_data_by_id(&role_id)?;
    if role.ResonantChainGroupId == 0 {
        return None;
    }
    for chain in &assets.tables.resonant_chain.items {
        if chain.GroupId == role.ResonantChainGroupId && chain.GroupIndex == 1 {
            return chain.ActivateConsume.keys().next().copied();
        }
    }
    None
}

fn push_updated_role_list(state: &mut State) -> Result<(), String> {
    let sex = state.player_components.basic.info.attributes.sex;
    let sel_id = state.player_components.basic.info.selected_main_role_id;
    let selected_main_role = role_helper::selected_main_role_id(
        &state.assets,
        &state.player_components.role,
        sex,
        sel_id,
    );
    let mut keys: Vec<i32> = state
        .player_components
        .role
        .role_map
        .keys()
        .copied()
        .filter(|&id| (1000..=1999).contains(&id))
        .collect();
    keys.sort_unstable();

    let mut role_list = Vec::with_capacity(keys.len());
    for role_id in keys {
        if let Some(selected) = selected_main_role
            && role_helper::is_main_role(&state.assets, role_id)
            && role_id != selected
        {
            continue;
        }
        let Some(rinfo) = state.player_components.role.role_map.get(&role_id).cloned() else {
            continue;
        };
        let pb_role = role_stats::to_client_role_info(
            &state.assets,
            &mut state.player_components.role,
            role_id,
            &rinfo,
            &state.player_components.weapon,
            &state.player_components.echo,
        )
        .map_err(|e| e.to_string())?;
        role_list.push(pb_role);
    }

    let _ = state.conn.push_msg(
        PbGetRoleListNotify::MSG_ID,
        &PbGetRoleListNotify {
            RoleList: role_list,
        },
    );
    Ok(())
}

fn first_pool_id(assets: &Assets, gacha_id: i32) -> i32 {
    let mut best_id = 0i32;
    let mut best_sort = i32::MAX;
    for pool in &assets.tables.gacha_pool.items {
        if pool.GachaId != gacha_id {
            continue;
        }
        if pool.Sort < best_sort || (pool.Sort == best_sort && (best_id == 0 || pool.Id < best_id))
        {
            best_id = pool.Id;
            best_sort = pool.Sort;
        }
    }
    best_id
}

fn gacha_rule_group_id(assets: &Assets, gacha_id: i32) -> i32 {
    if let Some(gacha) = assets.tables.gacha.get_data_by_id(&gacha_id)
        && gacha.RuleGroupId != 0
    {
        return gacha.RuleGroupId;
    }
    gacha_id
}

fn pool_belongs_to_gacha(assets: &Assets, gacha_id: i32, pool_id: i32) -> bool {
    assets
        .tables
        .gacha_pool
        .items
        .iter()
        .any(|p| p.GachaId == gacha_id && p.Id == pool_id)
}

fn selected_pool_view(assets: &Assets, banner: &mut Banner) -> Option<GachaViewInfo> {
    let pool_id = if banner.selected_pool_id != 0
        && pool_belongs_to_gacha(assets, banner.id, banner.selected_pool_id)
    {
        banner.selected_pool_id
    } else {
        first_pool_id(assets, banner.id)
    };
    if pool_id == 0 {
        return None;
    }
    banner.selected_pool_id = pool_id;
    assets
        .tables
        .gacha_view_info
        .get_data_by_id(&pool_id)
        .map(|v| v.into_owned())
}

fn pool_title(vt: i32) -> &'static str {
    match vt {
        view_type::NOVICE => "Novice Convene",
        view_type::FEATURED_ROLE => "Featured Resonator Convene",
        view_type::ANNIVERSARY_ROLE => "Anniversary Resonator Convene",
        view_type::NEW_VOYAGE_ROLE => "New Voyage Resonator Convene",
        view_type::COLLAB_ROLE => "Collab Resonator Convene",
        view_type::RERUN_ROLE => "Rerun Resonator Convene",
        view_type::FEATURED_WEAPON => "Featured Weapon Convene",
        view_type::ANNIVERSARY_WEAPON => "Anniversary Weapon Convene",
        view_type::NEW_VOYAGE_WEAPON => "New Voyage Weapon Convene",
        view_type::COLLAB_WEAPON => "Collab Weapon Convene",
        view_type::RERUN_WEAPON => "Rerun Weapon Convene",
        view_type::STANDARD_ROLE => "Standard Resonator Convene",
        view_type::STANDARD_WEAPON => "Standard Weapon Convene",
        view_type::BEGINNER_CHOICE => "Beginner's Choice Convene",
        _ => "Convene",
    }
}

fn pool_description(vt: i32) -> &'static str {
    match vt {
        view_type::NOVICE => "Novice Convene",
        view_type::FEATURED_ROLE
        | view_type::ANNIVERSARY_ROLE
        | view_type::NEW_VOYAGE_ROLE
        | view_type::COLLAB_ROLE
        | view_type::RERUN_ROLE => {
            "Every 10 Convenes guarantees a\n4-Star or above item.\nA 5-Star Character is guaranteed\nwithin 80 Convenes."
        }
        view_type::FEATURED_WEAPON => "Featured Weapon Convene Event",
        view_type::ANNIVERSARY_WEAPON => "Anniversary Weapon Convene Event",
        view_type::NEW_VOYAGE_WEAPON => "New Voyage Weapon Convene Event",
        view_type::COLLAB_WEAPON => "Collab Weapon Convene Event",
        view_type::RERUN_WEAPON => "Rerun Weapon Convene Event",
        view_type::STANDARD_ROLE => "Standard Resonator Convene",
        view_type::STANDARD_WEAPON => "Standard Weapon Convene",
        view_type::BEGINNER_CHOICE => "Beginner's Choice Convene",
        _ => "Convene Event",
    }
}

fn display_text(key: &str, fallback: &str) -> Vec<u8> {
    if key.starts_with("Text_") {
        fallback.as_bytes().to_vec()
    } else {
        key.as_bytes().to_vec()
    }
}

fn build_pool_info(view: &GachaViewInfo) -> GachaPoolInfo {
    GachaPoolInfo {
        Id: view.Id,
        BeginTime: 0,
        EndTime: 0,
        Title: display_text(&view.SummaryTitle, pool_title(view.Type)),
        Description: display_text(&view.SummaryDescribe, pool_description(view.Type)),
        UiType: view.Type,
        ThemeColor: view.ThemeColor.as_bytes().to_vec(),
        ShowIdList: view.ShowIdList.clone(),
        UpList: view.UpList.clone(),
        PreviewIdList: view.PreviewIdList.clone(),
        ComplianceDetail: Vec::new(),
    }
}

fn is_beginner_choice(gacha_id: i32, vt: i32) -> bool {
    vt == view_type::BEGINNER_CHOICE && gacha_id == 5
}

fn is_novice_banner(assets: &Assets, gacha_id: i32, vt: i32) -> bool {
    vt == view_type::NOVICE || gacha_rule_group_id(assets, gacha_id) == 5
}

fn banner_total_limit(assets: &Assets, gacha_id: i32, vt: i32) -> i32 {
    if is_beginner_choice(gacha_id, vt) {
        return 1;
    }
    if is_novice_banner(assets, gacha_id, vt) {
        return 50;
    }
    if vt == view_type::BEGINNER_CHOICE || gacha_rule_group_id(assets, gacha_id) == 6 {
        return 80;
    }
    0
}

fn consume_count_for_pull(assets: &Assets, gacha_id: i32, vt: i32, gacha_times: i32) -> Option<i32> {
    if is_beginner_choice(gacha_id, vt) {
        return (gacha_times == 1).then_some(1);
    }
    if is_novice_banner(assets, gacha_id, vt) {
        return (gacha_times == 10).then_some(8);
    }
    if gacha_times == 1 || gacha_times == 10 {
        Some(gacha_times)
    } else {
        None
    }
}

fn gacha_consumes(gacha_id: i32, vt: i32) -> Vec<GachaConsume> {
    if vt == view_type::NOVICE {
        vec![GachaConsume {
            Times: 10,
            Consume: 8,
        }]
    } else if is_beginner_choice(gacha_id, vt) {
        vec![GachaConsume {
            Times: 1,
            Consume: 1,
        }]
    } else {
        vec![
            GachaConsume {
                Times: 1,
                Consume: 1,
            },
            GachaConsume {
                Times: 10,
                Consume: 10,
            },
        ]
    }
}

fn resources_id(vt: i32) -> &'static str {
    match vt {
        view_type::FEATURED_ROLE
        | view_type::ANNIVERSARY_ROLE
        | view_type::NEW_VOYAGE_ROLE
        | view_type::COLLAB_ROLE
        | view_type::RERUN_ROLE => "UiItem_RoleUpGachaPool",
        view_type::FEATURED_WEAPON
        | view_type::ANNIVERSARY_WEAPON
        | view_type::NEW_VOYAGE_WEAPON
        | view_type::COLLAB_WEAPON
        | view_type::RERUN_WEAPON
        | view_type::STANDARD_WEAPON => "UiItem_WeaponGachaPool",
        view_type::NOVICE => "UiItem_NewPlayerGachaPool",
        view_type::BEGINNER_CHOICE => "UiItem_LuckdrawPixF",
        _ => "UiItem_BaseGachaPool",
    }
}

fn build_gacha_info(
    assets: &Assets,
    gacha_comp: &mut PlayerGachaComponent,
    gacha_id: i32,
) -> Option<PbGachaInfo> {
    let first_pool = first_pool_id(assets, gacha_id);
    let banner = gacha_comp.info.ensure_banner(gacha_id, first_pool);
    let view = selected_pool_view(assets, banner)?;
    let daily_pulls = banner.daily_pulls;
    let total_pulls = banner.total_pulls;
    let use_pool_id = banner.selected_pool_id;

    let mut pools = Vec::new();
    for pool in &assets.tables.gacha_pool.items {
        if pool.GachaId != gacha_id {
            continue;
        }
        if let Some(pv) = assets.tables.gacha_view_info.get_data_by_id(&pool.Id) {
            pools.push(build_pool_info(&pv));
        }
    }
    if pools.is_empty() {
        return None;
    }

    Some(PbGachaInfo {
        Id: gacha_id,
        TodayTimes: daily_pulls,
        TotalTimes: total_pulls,
        ItemId: inventory_helper::gacha_currency_item_id(gacha_id, view.Type),
        GachaConsumes: gacha_consumes(gacha_id, view.Type),
        UsePoolId: use_pool_id,
        Pools: pools,
        BeginTime: 0,
        EndTime: 0,
        DailyLimitTimes: 0,
        TotalLimitTimes: banner_total_limit(assets, gacha_id, view.Type),
        ResourcesId: resources_id(view.Type).as_bytes().to_vec(),
        ..Default::default()
    })
}

fn gacha_sort_value(assets: &Assets, gacha_id: i32) -> i32 {
    assets
        .tables
        .gacha
        .get_data_by_id(&gacha_id)
        .map_or(i32::MAX, |g| g.Sort)
}

fn gacha_view_type(assets: &Assets, gacha_id: i32) -> i32 {
    let pool_id = first_pool_id(assets, gacha_id);
    if pool_id == 0 {
        return 0;
    }
    assets
        .tables
        .gacha_view_info
        .get_data_by_id(&pool_id)
        .map_or(0, |v| v.Type)
}

fn gacha_family_rank(vt: i32) -> i32 {
    match vt {
        view_type::FEATURED_ROLE
        | view_type::ANNIVERSARY_ROLE
        | view_type::NEW_VOYAGE_ROLE
        | view_type::COLLAB_ROLE
        | view_type::RERUN_ROLE => 10,
        view_type::FEATURED_WEAPON
        | view_type::ANNIVERSARY_WEAPON
        | view_type::NEW_VOYAGE_WEAPON
        | view_type::COLLAB_WEAPON
        | view_type::RERUN_WEAPON => 20,
        view_type::NOVICE => 30,
        view_type::BEGINNER_CHOICE => 40,
        view_type::STANDARD_ROLE => 50,
        view_type::STANDARD_WEAPON => 60,
        _ => 100,
    }
}

fn gacha_sub_rank(vt: i32) -> i32 {
    match vt {
        view_type::COLLAB_ROLE | view_type::COLLAB_WEAPON => 1,
        _ => 0,
    }
}

fn cmp_gacha_info(assets: &Assets, lhs: &PbGachaInfo, rhs: &PbGachaInfo) -> Ordering {
    let lt = gacha_view_type(assets, lhs.Id);
    let rt = gacha_view_type(assets, rhs.Id);
    gacha_family_rank(lt)
        .cmp(&gacha_family_rank(rt))
        .then_with(|| gacha_sub_rank(lt).cmp(&gacha_sub_rank(rt)))
        .then_with(|| gacha_sort_value(assets, lhs.Id).cmp(&gacha_sort_value(assets, rhs.Id)))
        .then_with(|| lhs.Id.cmp(&rhs.Id))
}

pub fn handle_gacha_info_request(
    state: &mut State,
    _req: GachaInfoRequest,
) -> Result<GachaInfoResponse, String> {
    let mut infos = Vec::new();
    for gacha in &state.assets.tables.gacha.items {
        if let Some(info) = build_gacha_info(
            &state.assets,
            &mut state.player_components.gacha,
            gacha.Id,
        ) {
            infos.push(info);
        }
    }
    infos.sort_by(|a, b| cmp_gacha_info(&state.assets, a, b));
    save_gacha_info(&state.fs, &state.player_components.gacha)?;
    Ok(GachaInfoResponse {
        ErrorCode: Some(ErrorCode::Success),
        GachaInfos: infos,
        DailyTotalLeftTimes: -1,
        RecordId: Vec::new(),
    })
}

pub fn handle_gacha_use_pool_request(
    state: &mut State,
    req: GachaUsePoolRequest,
) -> Result<GachaUsePoolResponse, String> {
    if !pool_belongs_to_gacha(&state.assets, req.GachaId, req.PoolId) {
        return Ok(GachaUsePoolResponse {
            ErrorCode: Some(ErrorCode::ErrGachaPoolConfigNotFound),
        });
    }
    let banner = state
        .player_components
        .gacha
        .info
        .ensure_banner(req.GachaId, req.PoolId);
    banner.selected_pool_id = req.PoolId;
    save_gacha_info(&state.fs, &state.player_components.gacha)?;
    Ok(GachaUsePoolResponse {
        ErrorCode: Some(ErrorCode::Success),
    })
}

fn item_quality(assets: &Assets, item_id: i32) -> i32 {
    if let Some(role) = assets.tables.role_info.get_data_by_id(&item_id) {
        return role.QualityId;
    }
    if let Some(weapon) = assets.tables.weapon_conf.get_data_by_id(&item_id) {
        return weapon.QualityId;
    }
    0
}

fn has_gacha_texture(assets: &Assets, item_id: i32) -> bool {
    assets
        .tables
        .gacha_texture_info
        .get_data_by_id(&item_id)
        .is_some()
}

fn is_featured_resonator_view(vt: i32) -> bool {
    matches!(
        vt,
        view_type::FEATURED_ROLE
            | view_type::ANNIVERSARY_ROLE
            | view_type::NEW_VOYAGE_ROLE
            | view_type::COLLAB_ROLE
            | view_type::RERUN_ROLE
    )
}

fn is_featured_weapon_view(vt: i32) -> bool {
    matches!(
        vt,
        view_type::FEATURED_WEAPON
            | view_type::ANNIVERSARY_WEAPON
            | view_type::NEW_VOYAGE_WEAPON
            | view_type::COLLAB_WEAPON
            | view_type::RERUN_WEAPON
    )
}

fn is_featured_view(vt: i32) -> bool {
    is_featured_resonator_view(vt) || is_featured_weapon_view(vt)
}

fn item_matches_banner(assets: &Assets, item_id: i32, vt: i32, quality: i32) -> bool {
    let is_role = assets.tables.role_info.get_data_by_id(&item_id).is_some();
    let is_weapon = assets.tables.weapon_conf.get_data_by_id(&item_id).is_some();
    if !has_gacha_texture(assets, item_id) {
        return false;
    }
    if quality < 5 {
        return is_role || is_weapon;
    }
    match vt {
        view_type::FEATURED_ROLE
        | view_type::STANDARD_ROLE
        | view_type::ANNIVERSARY_ROLE
        | view_type::NEW_VOYAGE_ROLE
        | view_type::COLLAB_ROLE
        | view_type::RERUN_ROLE
        | view_type::NOVICE
        | view_type::BEGINNER_CHOICE => is_role,
        view_type::FEATURED_WEAPON
        | view_type::STANDARD_WEAPON
        | view_type::ANNIVERSARY_WEAPON
        | view_type::NEW_VOYAGE_WEAPON
        | view_type::COLLAB_WEAPON
        | view_type::RERUN_WEAPON => is_weapon,
        _ => is_role || is_weapon,
    }
}

#[allow(clippy::too_many_arguments)]
fn append_detail_items_excluding(
    assets: &Assets,
    list: &mut Vec<GachaItem>,
    values: &[i32],
    exclude_a: &[i32],
    exclude_b: &[i32],
    vt: i32,
    quality: i32,
    is_up: bool,
) {
    for &item_id in values {
        if exclude_a.contains(&item_id) || exclude_b.contains(&item_id) {
            continue;
        }
        if list.iter().any(|i| i.ItemId == item_id) {
            continue;
        }
        if item_quality(assets, item_id) == quality
            && item_matches_banner(assets, item_id, vt, quality)
        {
            list.push(GachaItem {
                ItemId: item_id,
                IsUp: is_up,
            });
        }
    }
}

fn append_quality_items_excluding(
    assets: &Assets,
    list: &mut Vec<i32>,
    values: &[i32],
    exclude_a: &[i32],
    exclude_b: &[i32],
    vt: i32,
    quality: i32,
) {
    for &item_id in values {
        if exclude_a.contains(&item_id) || exclude_b.contains(&item_id) || list.contains(&item_id) {
            continue;
        }
        if item_quality(assets, item_id) == quality
            && item_matches_banner(assets, item_id, vt, quality)
        {
            list.push(item_id);
        }
    }
}

fn append_fallback_quality_items(
    assets: &Assets,
    list: &mut Vec<i32>,
    vt: i32,
    quality: i32,
    exclude_a: &[i32],
    exclude_b: &[i32],
) {
    if quality == 3 {
        for weapon in &assets.tables.weapon_conf.items {
            if !exclude_a.contains(&weapon.ItemId)
                && !exclude_b.contains(&weapon.ItemId)
                && !list.contains(&weapon.ItemId)
                && weapon.QualityId == quality
                && has_gacha_texture(assets, weapon.ItemId)
            {
                list.push(weapon.ItemId);
            }
        }
        return;
    }

    if quality == 4 {
        for role in &assets.tables.role_info.items {
            if !exclude_a.contains(&role.Id)
                && !exclude_b.contains(&role.Id)
                && !list.contains(&role.Id)
                && !role.IsTrial
                && item_quality(assets, role.Id) == quality
                && has_gacha_texture(assets, role.Id)
            {
                list.push(role.Id);
            }
        }
        for weapon in &assets.tables.weapon_conf.items {
            if !exclude_a.contains(&weapon.ItemId)
                && !exclude_b.contains(&weapon.ItemId)
                && !list.contains(&weapon.ItemId)
                && weapon.QualityId == quality
                && has_gacha_texture(assets, weapon.ItemId)
            {
                list.push(weapon.ItemId);
            }
        }
        return;
    }

    match vt {
        view_type::FEATURED_ROLE
        | view_type::STANDARD_ROLE
        | view_type::ANNIVERSARY_ROLE
        | view_type::NEW_VOYAGE_ROLE
        | view_type::COLLAB_ROLE
        | view_type::RERUN_ROLE
        | view_type::NOVICE
        | view_type::BEGINNER_CHOICE => {
            for role in &assets.tables.role_info.items {
                if !exclude_a.contains(&role.Id)
                    && !exclude_b.contains(&role.Id)
                    && !list.contains(&role.Id)
                    && !role.IsTrial
                    && item_quality(assets, role.Id) == quality
                    && has_gacha_texture(assets, role.Id)
                {
                    list.push(role.Id);
                }
            }
        }
        view_type::FEATURED_WEAPON
        | view_type::STANDARD_WEAPON
        | view_type::ANNIVERSARY_WEAPON
        | view_type::NEW_VOYAGE_WEAPON
        | view_type::COLLAB_WEAPON
        | view_type::RERUN_WEAPON => {
            for weapon in &assets.tables.weapon_conf.items {
                if !exclude_a.contains(&weapon.ItemId)
                    && !exclude_b.contains(&weapon.ItemId)
                    && !list.contains(&weapon.ItemId)
                    && weapon.QualityId == quality
                    && has_gacha_texture(assets, weapon.ItemId)
                {
                    list.push(weapon.ItemId);
                }
            }
        }
        _ => {}
    }
}

fn build_up_detail_list(assets: &Assets, view: &GachaViewInfo, quality: i32) -> Vec<GachaItem> {
    let mut list = Vec::new();
    if !is_featured_view(view.Type) {
        return list;
    }
    append_detail_items_excluding(assets, &mut list, &view.ShowIdList, &[], &[], view.Type, quality, true);
    append_detail_items_excluding(assets, &mut list, &view.UpList, &[], &[], view.Type, quality, true);
    list
}

fn build_regular_detail_list(assets: &Assets, view: &GachaViewInfo, quality: i32) -> Vec<GachaItem> {
    let mut list = Vec::new();
    let exclude_show: &[i32] = if is_featured_view(view.Type) && quality >= 4 {
        &view.ShowIdList
    } else {
        &[]
    };
    let exclude_up: &[i32] = if is_featured_view(view.Type) && quality >= 4 {
        &view.UpList
    } else {
        &[]
    };
    append_detail_items_excluding(
        assets,
        &mut list,
        &view.PreviewIdList,
        exclude_show,
        exclude_up,
        view.Type,
        quality,
        false,
    );
    if list.is_empty() {
        let mut fallback = Vec::new();
        append_fallback_quality_items(
            assets,
            &mut fallback,
            view.Type,
            quality,
            exclude_show,
            exclude_up,
        );
        for item_id in fallback {
            list.push(GachaItem {
                ItemId: item_id,
                IsUp: false,
            });
        }
    }
    list
}

fn build_configured_detail_list(
    assets: &Assets,
    view: &GachaViewInfo,
    quality: i32,
    is_up: bool,
) -> Vec<GachaItem> {
    let mut list = Vec::new();
    append_detail_items_excluding(assets, &mut list, &view.ShowIdList, &[], &[], view.Type, quality, is_up);
    append_detail_items_excluding(assets, &mut list, &view.UpList, &[], &[], view.Type, quality, is_up);
    append_detail_items_excluding(
        assets,
        &mut list,
        &view.PreviewIdList,
        &[],
        &[],
        view.Type,
        quality,
        is_up,
    );
    list
}

fn build_standard_five_star_resonators(assets: &Assets) -> Vec<i32> {
    let mut candidates = Vec::new();
    for view in &assets.tables.gacha_view_info.items {
        if view.Type != view_type::STANDARD_ROLE {
            continue;
        }
        append_quality_items_excluding(
            assets,
            &mut candidates,
            &view.PreviewIdList,
            &[],
            &[],
            view.Type,
            5,
        );
        if !candidates.is_empty() {
            break;
        }
    }
    candidates
}

pub fn handle_gacha_pool_detail_request(
    state: &mut State,
    req: GachaPoolDetailRequest,
) -> Result<GachaPoolDetailResponse, String> {
    let Some(view) = state
        .assets
        .tables
        .gacha_view_info
        .get_data_by_id(&req.PoolId)
    else {
        return Ok(GachaPoolDetailResponse {
            ErrorCode: Some(ErrorCode::ErrGachaPoolConfigNotFound),
            GachaPoolDetail: None,
        });
    };

    let five_roles = if is_featured_resonator_view(view.Type) {
        let mut list = build_up_detail_list(&state.assets, &view, 5);
        for item_id in build_standard_five_star_resonators(&state.assets) {
            if !list.iter().any(|item| item.ItemId == item_id) {
                list.push(GachaItem {
                    ItemId: item_id,
                    IsUp: false,
                });
            }
        }
        list
    } else if matches!(
        view.Type,
        view_type::STANDARD_ROLE | view_type::NOVICE | view_type::BEGINNER_CHOICE
    ) {
        build_configured_detail_list(&state.assets, &view, 5, false)
    } else {
        Vec::new()
    };

    let five_weapons = if is_featured_weapon_view(view.Type) {
        build_up_detail_list(&state.assets, &view, 5)
    } else if view.Type == view_type::STANDARD_WEAPON {
        build_configured_detail_list(&state.assets, &view, 5, false)
    } else {
        Vec::new()
    };

    let three_roles = {
        let mut list = Vec::new();
        append_detail_items_excluding(
            &state.assets,
            &mut list,
            &view.PreviewIdList,
            &[],
            &[],
            view.Type,
            3,
            false,
        );
        if list.is_empty() {
            let mut fb = Vec::new();
            append_fallback_quality_items(&state.assets, &mut fb, view.Type, 3, &[], &[]);
            list = fb
                .into_iter()
                .map(|item_id| GachaItem {
                    ItemId: item_id,
                    IsUp: false,
                })
                .collect();
        }
        list
    };

    Ok(GachaPoolDetailResponse {
        ErrorCode: Some(ErrorCode::Success),
        GachaPoolDetail: Some(GachaPoolDetail {
            Text: display_text(&view.SummaryDescribe, pool_description(view.Type)),
            FiveStarRoles: five_roles,
            FiveStarWeapons: five_weapons,
            FourStarRoles: build_up_detail_list(&state.assets, &view, 4),
            FourStarWeapons: build_regular_detail_list(&state.assets, &view, 4),
            ThreeStarRoles: three_roles,
            FiveStarTitle: b"5-Star Convene Items".to_vec(),
            FileStarDetail: b"Featured 5-Star".to_vec(),
            FourStarTitle: b"4-Star Convene Items".to_vec(),
            FourStarDetail: b"Featured 4-Star".to_vec(),
            ThreeStarTitle: b"3-Star Convene Items".to_vec(),
            ThreeStarDetail: b"Available 3-Star".to_vec(),
        }),
    })
}

fn secure_random_less_than(less_than: u64) -> Result<u64, String> {
    if less_than == 0 {
        return Ok(0);
    }
    let bound = u128::from(less_than);
    let range = 1u128 << 64;
    let limit = range - (range % bound);
    let mut buf = [0u8; 8];
    loop {
        getrandom::fill(&mut buf).map_err(|e| format!("rng failed: {e}"))?;
        let val = u128::from(u64::from_le_bytes(buf));
        if val < limit {
            return Ok((val % bound) as u64);
        }
    }
}

fn roll(less_than: i32) -> Result<i32, String> {
    Ok(secure_random_less_than(less_than as u64)? as i32)
}

fn choose_from(values: &[i32]) -> Result<i32, String> {
    if values.is_empty() {
        return Ok(0);
    }
    let idx = secure_random_less_than(values.len() as u64)? as usize;
    Ok(values[idx])
}

fn roll_ratio(numerator: i32, denominator: i32) -> Result<bool, String> {
    if denominator <= 0 || numerator <= 0 {
        return Ok(false);
    }
    if numerator >= denominator {
        return Ok(true);
    }
    Ok(roll(denominator)? < numerator)
}

fn five_star_hard_pity(vt: i32) -> i32 {
    let rates = &GameConfig::embedded().gacha_rates;
    if vt == view_type::NOVICE {
        rates.novice.five_star_hard_pity_pull as i32
    } else {
        rates.normal.five_star_hard_pity_pull as i32
    }
}

fn normal_five_star_rate(pull_number: i32) -> i32 {
    let r = &GameConfig::embedded().gacha_rates;
    let base = r.five_star_base_rate as i32;
    let n = &r.normal;
    if pull_number >= n.five_star_soft_pity_force_pull as i32 {
        return r.rate_denominator as i32;
    }
    let start = n.five_star_soft_pity_start_pull as i32;
    if pull_number < start {
        return base;
    }
    let p1_end = n.five_star_soft_pity_phase_one_end_pull as i32;
    let p1_step = n.five_star_soft_pity_phase_one_step as i32;
    if pull_number <= p1_end {
        return base + p1_step * (pull_number - start + 1);
    }
    let p1_rate = base + p1_step * (p1_end - start + 1);
    let p2_end = n.five_star_soft_pity_phase_two_end_pull as i32;
    let p2_step = n.five_star_soft_pity_phase_two_step as i32;
    if pull_number <= p2_end {
        return p1_rate + p2_step * (pull_number - p1_end);
    }
    let p2_rate = p1_rate + p2_step * (p2_end - p1_end);
    let p3_step = n.five_star_soft_pity_phase_three_step as i32;
    p2_rate + p3_step * (pull_number - p2_end)
}

fn novice_five_star_rate(pull_number: i32) -> i32 {
    let r = &GameConfig::embedded().gacha_rates;
    let base = r.five_star_base_rate as i32;
    let start = r.novice.five_star_soft_pity_start_pull as i32;
    if pull_number < start {
        return base;
    }
    let step = r.novice.five_star_soft_pity_step as i32;
    (base + step * (pull_number - start + 1)).min(r.rate_denominator as i32)
}

fn determine_rarity(bucket: &Bucket, gacha_id: i32, vt: i32) -> Result<i32, String> {
    if is_beginner_choice(gacha_id, vt) {
        return Ok(5);
    }
    let pull_number = bucket.pull_count + 1;
    if pull_number >= five_star_hard_pity(vt) {
        return Ok(5);
    }
    if bucket.pity_four + 1 >= 10 {
        return Ok(4);
    }
    let five_rate = if vt == view_type::NOVICE {
        novice_five_star_rate(pull_number)
    } else {
        normal_five_star_rate(pull_number)
    };
    let rates = &GameConfig::embedded().gacha_rates;
    let value = roll(rates.rate_denominator as i32)?;
    if value < five_rate {
        return Ok(5);
    }
    if value < five_rate + rates.four_star_base_rate as i32 {
        return Ok(4);
    }
    Ok(3)
}

fn update_pity(bucket: &mut Bucket, banner: &mut Banner, rarity: i32) {
    if rarity == 5 {
        bucket.pull_count = 0;
        bucket.pity_four += 1;
    } else {
        bucket.pull_count += 1;
        bucket.pity_four = if rarity == 4 { 0 } else { bucket.pity_four + 1 };
    }
    banner.daily_pulls += 1;
    banner.total_pulls += 1;
}

fn build_featured_reward_candidates(
    assets: &Assets,
    view: &GachaViewInfo,
    quality: i32,
) -> Vec<i32> {
    let mut candidates = Vec::new();
    if !is_featured_view(view.Type) {
        return candidates;
    }
    if quality == 5 {
        append_quality_items_excluding(assets, &mut candidates, &view.ShowIdList, &[], &[], view.Type, quality);
        append_quality_items_excluding(assets, &mut candidates, &view.UpList, &[], &[], view.Type, quality);
    } else if quality == 4 {
        append_quality_items_excluding(assets, &mut candidates, &view.UpList, &[], &[], view.Type, quality);
    }
    candidates
}

fn build_regular_reward_candidates(assets: &Assets, view: &GachaViewInfo, quality: i32) -> Vec<i32> {
    let mut candidates = Vec::new();
    let exclude_show: &[i32] = if is_featured_view(view.Type) && quality >= 4 {
        &view.ShowIdList
    } else {
        &[]
    };
    let exclude_up: &[i32] = if is_featured_view(view.Type) && quality >= 4 {
        &view.UpList
    } else {
        &[]
    };
    append_quality_items_excluding(
        assets,
        &mut candidates,
        &view.PreviewIdList,
        exclude_show,
        exclude_up,
        view.Type,
        quality,
    );
    if candidates.is_empty() {
        append_fallback_quality_items(
            assets,
            &mut candidates,
            view.Type,
            quality,
            exclude_show,
            exclude_up,
        );
    }
    candidates
}

fn build_reward_candidates(assets: &Assets, view: &GachaViewInfo, quality: i32) -> Vec<i32> {
    if is_featured_view(view.Type) && quality == 5 {
        let mut candidates = build_featured_reward_candidates(assets, view, quality);
        if candidates.is_empty() {
            append_fallback_quality_items(assets, &mut candidates, view.Type, quality, &[], &[]);
        }
        return candidates;
    }
    build_regular_reward_candidates(assets, view, quality)
}

fn choose_featured_resonator_five_star(
    assets: &Assets,
    view: &GachaViewInfo,
    bucket: &mut Bucket,
) -> Result<i32, Option<ErrorCode>> {
    let featured = build_reward_candidates(assets, view, 5);
    if featured.is_empty() {
        return Err(Some(ErrorCode::ErrGachaPoolConfigNotFound));
    }
    let rates = &GameConfig::embedded().gacha_rates;
    let won = bucket.guarantee_five
        || roll_ratio(
            rates.featured_five_star_win_numerator as i32,
            rates.featured_five_star_win_denominator as i32,
        )
        .map_err(|_| None)?;
    if won {
        bucket.guarantee_five = false;
        return choose_from(&featured).map_err(|_| None);
    }
    let standard = build_standard_five_star_resonators(assets);
    if standard.is_empty() {
        bucket.guarantee_five = false;
        return choose_from(&featured).map_err(|_| None);
    }
    bucket.guarantee_five = true;
    choose_from(&standard).map_err(|_| None)
}

fn choose_featured_four_star(
    assets: &Assets,
    view: &GachaViewInfo,
    bucket: &mut Bucket,
) -> Result<i32, Option<ErrorCode>> {
    let up = build_featured_reward_candidates(assets, view, 4);
    if up.is_empty() {
        let regular = build_regular_reward_candidates(assets, view, 4);
        if regular.is_empty() {
            return Err(Some(ErrorCode::ErrGachaPoolConfigNotFound));
        }
        return choose_from(&regular).map_err(|_| None);
    }
    let rates = &GameConfig::embedded().gacha_rates;
    let won = bucket.guarantee_four
        || roll_ratio(
            rates.featured_four_star_win_numerator as i32,
            rates.featured_four_star_win_denominator as i32,
        )
        .map_err(|_| None)?;
    if won {
        bucket.guarantee_four = false;
        return choose_from(&up).map_err(|_| None);
    }
    let regular = build_regular_reward_candidates(assets, view, 4);
    if regular.is_empty() {
        bucket.guarantee_four = false;
        return choose_from(&up).map_err(|_| None);
    }
    bucket.guarantee_four = true;
    choose_from(&regular).map_err(|_| None)
}

fn choose_reward(
    assets: &Assets,
    view: &GachaViewInfo,
    bucket: &mut Bucket,
    rarity: i32,
) -> Result<i32, Option<ErrorCode>> {
    if view.Type == view_type::BEGINNER_CHOICE && rarity == 5 && !view.ShowIdList.is_empty() {
        return Ok(view.ShowIdList[0]);
    }
    if rarity == 5 && is_featured_resonator_view(view.Type) {
        return choose_featured_resonator_five_star(assets, view, bucket);
    }
    if rarity == 4 && is_featured_view(view.Type) {
        return choose_featured_four_star(assets, view, bucket);
    }
    let candidates = build_reward_candidates(assets, view, rarity);
    if candidates.is_empty() {
        return Err(Some(ErrorCode::ErrGachaPoolConfigNotFound));
    }
    choose_from(&candidates).map_err(|_| None)
}

fn max_resonant_chain_index(assets: &Assets, role_id: i32) -> i32 {
    let Some(role) = assets.tables.role_info.get_data_by_id(&role_id) else {
        return 6;
    };
    if role.ResonantChainGroupId == 0 {
        return 6;
    }
    let max_idx = assets
        .tables
        .resonant_chain
        .items
        .iter()
        .filter(|c| c.GroupId == role.ResonantChainGroupId)
        .map(|c| c.GroupIndex)
        .max()
        .unwrap_or(0);
    if max_idx > 0 { max_idx } else { 6 }
}

fn record_updated_normal_item(
    added_items: &mut Vec<NormalItem>,
    updated_items: &mut Vec<NormalItem>,
    old_count: i32,
    inventory_comp: &PlayerInventoryComponent,
    item_id: i32,
) {
    let new_count = inventory_comp.info.normal_item_count(item_id);
    let target = if old_count <= 0 {
        added_items
    } else {
        updated_items
    };
    if let Some(entry) = target.iter_mut().find(|e| e.Id == item_id) {
        entry.Count = new_count;
    } else {
        target.push(NormalItem {
            Id: item_id,
            Count: new_count,
            ExpireTime: 0,
        });
    }
}

fn grant_bonus_item(
    inventory_comp: &mut PlayerInventoryComponent,
    reward_list: &mut Vec<GachaReward>,
    added_items: &mut Vec<NormalItem>,
    updated_items: &mut Vec<NormalItem>,
    item_id: i32,
    count: i32,
) {
    if item_id <= 0 || count <= 0 {
        return;
    }
    let old_count = inventory_comp.info.normal_item_count(item_id);
    inventory_comp.info.add_normal_item(item_id, count);
    reward_list.push(GachaReward {
        ItemId: item_id,
        ItemCount: count,
    });
    record_updated_normal_item(added_items, updated_items, old_count, inventory_comp, item_id);
}

pub fn handle_gacha_request(state: &mut State, req: GachaRequest) -> Result<GachaResponse, String> {
    let first_pool = first_pool_id(&state.assets, req.GachaId);
    let rule_group = gacha_rule_group_id(&state.assets, req.GachaId);
    let (mut next_banner, view) = {
        let banner = state
            .player_components
            .gacha
            .info
            .ensure_banner(req.GachaId, first_pool);
        let Some(view) = selected_pool_view(&state.assets, banner) else {
            return Ok(GachaResponse {
                ErrorCode: Some(ErrorCode::ErrGachaConfigNotFound),
                GachaResults: Vec::new(),
            });
        };
        (banner.clone(), view)
    };
    let mut next_bucket = state
        .player_components
        .gacha
        .info
        .ensure_bucket(rule_group)
        .clone();

    let Some(consume_count) =
        consume_count_for_pull(&state.assets, req.GachaId, view.Type, req.GachaTimes)
    else {
        return Ok(GachaResponse {
            ErrorCode: Some(ErrorCode::ErrGachaTimesNonsupport),
            GachaResults: Vec::new(),
        });
    };

    let total_limit = banner_total_limit(&state.assets, req.GachaId, view.Type);
    if total_limit > 0 && next_banner.total_pulls + req.GachaTimes > total_limit {
        return Ok(GachaResponse {
            ErrorCode: Some(ErrorCode::ErrGachaTotalTimesLimit),
            GachaResults: Vec::new(),
        });
    }

    let currency_id = inventory_helper::gacha_currency_item_id(req.GachaId, view.Type);
    let costs = [CostItem {
        id: currency_id,
        count: consume_count,
    }];
    if !consume::has_costs(
        &state.player_components.basic,
        &state.player_components.inventory,
        &costs,
    ) {
        return Ok(GachaResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            GachaResults: Vec::new(),
        });
    }

    let mut rolled = Vec::with_capacity(req.GachaTimes.max(0) as usize);
    for _ in 0..req.GachaTimes.max(0) {
        let rarity = determine_rarity(&next_bucket, req.GachaId, view.Type)?;
        let reward_id = match choose_reward(&state.assets, &view, &mut next_bucket, rarity) {
            Ok(id) => id,
            Err(Some(code)) => {
                return Ok(GachaResponse {
                    ErrorCode: Some(code),
                    GachaResults: Vec::new(),
                });
            }
            Err(None) => return Err("gacha rng failed".to_string()),
        };
        update_pity(&mut next_bucket, &mut next_banner, rarity);
        rolled.push(reward_id);
    }

    let consumed = consume::consume_costs(
        &mut state.conn,
        &state.fs,
        &mut state.player_components.basic,
        &mut state.player_components.inventory,
        &costs,
    )
    .map_err(|e| e.to_string())?;
    if !consumed {
        return Ok(GachaResponse {
            ErrorCode: Some(ErrorCode::ErrItemNotEnough),
            GachaResults: Vec::new(),
        });
    }

    *state
        .player_components
        .gacha
        .info
        .ensure_bucket(rule_group) = next_bucket;
    *state
        .player_components
        .gacha
        .info
        .ensure_banner(req.GachaId, first_pool) = next_banner;

    let mut results = Vec::with_capacity(rolled.len());
    let mut added_weapons: Vec<PbWeaponItem> = Vec::new();
    let mut starter_weapons: Vec<PbWeaponItem> = Vec::new();
    let mut added_items: Vec<NormalItem> = Vec::new();
    let mut updated_items: Vec<NormalItem> = Vec::new();
    let mut has_inventory_updates = false;
    let mut has_new_roles = false;

    for reward_id in rolled {
        let mut gacha_result = GachaResult {
            GachaReward: Some(GachaReward {
                ItemId: reward_id,
                ItemCount: 1,
            }),
            ..Default::default()
        };

        if state
            .assets
            .tables
            .weapon_conf
            .get_data_by_id(&reward_id)
            .is_some()
        {
            let incr_id =
                special_item_incr::next(&state.fs, state.player_components.weapon.player_id)
                    .map_err(|e| e.to_string())?;
            let weapon = WeaponItem {
                id: reward_id,
                func_value: WeaponItem::default_func_value(&state.assets, reward_id),
                ..Default::default()
            };
            let proto = weapon.to_proto(incr_id);
            save_weapon_info(
                &state.fs,
                &state.player_components.weapon,
                incr_id,
                &weapon,
            )?;
            state
                .player_components
                .weapon
                .weapon_map
                .insert(incr_id, weapon);
            added_weapons.push(proto);

            let quality = item_quality(&state.assets, reward_id);
            let (bonus_id, bonus_cnt) = if quality >= 5 {
                (ITEM_ID_AFTERGLOW_CORAL, 15)
            } else if quality == 4 {
                (ITEM_ID_AFTERGLOW_CORAL, 3)
            } else {
                (ITEM_ID_OSCILLATED_CORAL, 15)
            };
            grant_bonus_item(
                &mut state.player_components.inventory,
                &mut gacha_result.ExtraRewards,
                &mut added_items,
                &mut updated_items,
                bonus_id,
                bonus_cnt,
            );
            has_inventory_updates = true;
        } else if state
            .assets
            .tables
            .role_info
            .get_data_by_id(&reward_id)
            .is_some()
        {
            if let Some(existing) = state.player_components.role.role_map.get(&reward_id).cloned()
            {
                let is_five_star = item_quality(&state.assets, reward_id) >= 5;
                let waveband_opt = role_waveband_item(&state.assets, reward_id);
                let owned_wavebands = waveband_opt.map_or(0, |wb| {
                    state.player_components.inventory.info.normal_item_count(wb)
                });
                let max_chain = max_resonant_chain_index(&state.assets, reward_id);
                if existing.resonant_chain_group_index + owned_wavebands < max_chain {
                    if let Some(waveband_id) = waveband_opt {
                        grant_bonus_item(
                            &mut state.player_components.inventory,
                            &mut gacha_result.TransformRewards,
                            &mut added_items,
                            &mut updated_items,
                            waveband_id,
                            1,
                        );
                    }
                    let dup_coral = if is_five_star { 15 } else { 3 };
                    grant_bonus_item(
                        &mut state.player_components.inventory,
                        &mut gacha_result.TransformRewards,
                        &mut added_items,
                        &mut updated_items,
                        ITEM_ID_AFTERGLOW_CORAL,
                        dup_coral,
                    );
                } else {
                    let overflow_coral = if is_five_star { 45 } else { 8 };
                    grant_bonus_item(
                        &mut state.player_components.inventory,
                        &mut gacha_result.TransformRewards,
                        &mut added_items,
                        &mut updated_items,
                        ITEM_ID_AFTERGLOW_CORAL,
                        overflow_coral,
                    );
                }
                has_inventory_updates = true;
            } else {
                let new_role =
                    RoleInfo::create_default(&state.assets, reward_id).map_err(|e| e.to_string())?;
                let role_path = format!(
                    "player/{}/role/{}",
                    state.player_components.role.player_id, reward_id
                );
                comp_util::save_struct(&state.fs, &new_role, &role_path)
                    .map_err(|e| e.to_string())?;
                state
                    .player_components
                    .role
                    .role_map
                    .insert(reward_id, new_role);
                if role_helper::ensure_role_weapon(
                    &state.fs,
                    &state.assets,
                    &mut state.player_components.role,
                    &mut state.player_components.weapon,
                    reward_id,
                )
                .unwrap_or(false)
                    && let Some(rinfo) = state.player_components.role.role_map.get(&reward_id)
                    && let Some(witem) = state
                        .player_components
                        .weapon
                        .weapon_map
                        .get(&rinfo.weapon)
                {
                    starter_weapons.push(witem.to_proto(rinfo.weapon));
                }
                has_new_roles = true;
            }
        } else {
            let old_cnt = state
                .player_components
                .inventory
                .info
                .normal_item_count(reward_id);
            state
                .player_components
                .inventory
                .info
                .add_normal_item(reward_id, 1);
            record_updated_normal_item(
                &mut added_items,
                &mut updated_items,
                old_cnt,
                &state.player_components.inventory,
                reward_id,
            );
            has_inventory_updates = true;
        }

        results.push(gacha_result);
    }

    save_gacha_info(&state.fs, &state.player_components.gacha)?;

    if !added_weapons.is_empty() {
        let _ = state.conn.push_msg(
            WeaponItemAddNotify::MSG_ID,
            &WeaponItemAddNotify {
                WeaponItemList: added_weapons,
                AddFromRole: false,
                Reason: 0,
            },
        );
    }
    if !starter_weapons.is_empty() {
        let _ = state.conn.push_msg(
            WeaponItemAddNotify::MSG_ID,
            &WeaponItemAddNotify {
                WeaponItemList: starter_weapons,
                AddFromRole: true,
                Reason: 0,
            },
        );
    }
    if has_inventory_updates {
        save_inventory_info(&state.fs, &state.player_components.inventory)?;
        if !added_items.is_empty() {
            let _ = state.conn.push_msg(
                NormalItemAddNotify::MSG_ID,
                &NormalItemAddNotify {
                    NormalItemList: added_items,
                    NoTips: false,
                    Reason: 0,
                },
            );
        }
        if !updated_items.is_empty() {
            let _ = state.conn.push_msg(
                NormalItemUpdateNotify::MSG_ID,
                &NormalItemUpdateNotify {
                    NormalItemList: updated_items,
                    NoTips: false,
                },
            );
        }
    }
    if has_new_roles {
        push_updated_role_list(state)?;
    }

    Ok(GachaResponse {
        ErrorCode: Some(ErrorCode::Success),
        GachaResults: results,
    })
}
