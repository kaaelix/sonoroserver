use serde::Deserialize;
use sonoro_common::fs::FileSystem;
use sonoro_gamesv_data::Assets;

#[derive(Debug, Deserialize, Default)]
#[allow(non_snake_case)]
struct AutoPilotCircle {
    #[serde(default)]
    MapId: i32,
    #[serde(default)]
    WaySplines: Vec<i32>,
}

pub fn road_ids_for_map(assets: &Assets, fs: &FileSystem, map_id: i32) -> Result<Vec<i32>, String> {
    let circles: Vec<AutoPilotCircle> = if let Some(raw) = assets.read_raw_file("AutoPilotCircles") {
        serde_json::from_str(&raw).map_err(|e| e.to_string())?
    } else {
        let bytes = fs
            .read_file("assets/BinData/AutoPilotCircles.json")
            .map_err(|e| e.to_string())?
            .ok_or_else(|| "AutoPilotCircles not found in assets.db or BinData".to_string())?;
        serde_json::from_slice(&bytes).map_err(|e| e.to_string())?
    };

    let mut roads = Vec::new();
    for circle in circles {
        if circle.MapId != map_id {
            continue;
        }
        for road_id in circle.WaySplines {
            if !roads.contains(&road_id) {
                roads.push(road_id);
            }
        }
    }
    Ok(roads)
}
