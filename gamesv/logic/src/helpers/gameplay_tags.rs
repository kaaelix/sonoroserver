use sonoro_gamesv_data::data_tables::Table;
use sonoro_gamesv_data::tables::gameplay_tag_parent::GameplayTagParent;
use sonoro_gamesv_data::tables::gameplay_tag_remap::GameplayTagRemap;

const MAX_PARENT_DEPTH: usize = 32;
const FNV_PRIME: u32 = 16_777_619;

/// FNV-1a over UTF-16 code units matching UE/Kuro gameplay tag ID hashing,
/// mapped through `GameplayTagRemap` (returning the raw hash ID when unmapped).
pub fn id_from_name(remaps: &Table<GameplayTagRemap, i32>, name: &str) -> Option<i32> {
    let mut hash: u32 = 0;
    for unit in name.encode_utf16() {
        hash = (hash ^ u32::from(unit)).wrapping_mul(FNV_PRIME);
    }

    let raw_id = hash as i32;
    Some(
        remaps
            .get_data_by_id(&raw_id)
            .map_or(raw_id, |entry| entry.CanonicalId),
    )
}

pub fn contains(parents: &Table<GameplayTagParent, i32>, actual_id: i32, expected_id: i32) -> bool {
    if actual_id == expected_id {
        return true;
    }
    let mut current_id = actual_id;
    let mut remaining = MAX_PARENT_DEPTH;
    while current_id != 0 && remaining > 0 {
        if current_id == expected_id {
            return true;
        }
        let Some(entry) = parents.get_data_by_id(&current_id) else {
            return false;
        };
        current_id = entry.ParentId;
        remaining -= 1;
    }
    false
}
