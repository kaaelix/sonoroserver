use sonoro_gamesv_data::Assets;
use sonoro_gamesv_fs::progress_info::ProgressInfo;
use sonoro_proto::pb::{
    MoveSplineConfig, MoveSplineConfigCircleOneof, MoveSplineConfigCycleOneof,
    MoveSplineConfigEndPointOneof, MoveSplineConfigLookDirOneof, MoveSplineConfigStartPointOneof,
};

const ABSENT_VALUE: i64 = i64::MIN;

pub fn persist_move_spline_config(
    info: &mut ProgressInfo,
    config_id: i32,
    config_opt: Option<&MoveSplineConfig>,
) {
    if config_id <= 0 {
        return;
    }
    let default_cfg = MoveSplineConfig::default();
    let config = config_opt.unwrap_or(&default_cfg);

    let start = match config.StartPoint {
        Some(MoveSplineConfigStartPointOneof::StartPointIndex(v)) => i64::from(v),
        None => ABSENT_VALUE,
    };
    let end = match config.EndPoint {
        Some(MoveSplineConfigEndPointOneof::EndPointIndex(v)) => i64::from(v),
        None => ABSENT_VALUE,
    };
    let look_dir = match config.LookDir {
        Some(MoveSplineConfigLookDirOneof::IsLookDir(v)) => i64::from(i32::from(v)),
        None => ABSENT_VALUE,
    };
    let cycle = match config.Cycle {
        Some(MoveSplineConfigCycleOneof::CycleCount(v)) => i64::from(v),
        None => ABSENT_VALUE,
    };
    let circle = match config.Circle {
        Some(MoveSplineConfigCircleOneof::IsCircle(v)) => i64::from(i32::from(v)),
        None => ABSENT_VALUE,
    };

    for (field, value) in [
        ("start", start),
        ("end", end),
        ("look_dir", look_dir),
        ("cycle", cycle),
        ("circle", circle),
    ] {
        info.set_key_int(&format!("scene_item_spline_opt:{config_id}:{field}"), value);
    }
    info.set_key_int(&format!("scene_item_spline_opt:{config_id}:written"), 1);
}

pub fn restore_move_spline_config(info: &ProgressInfo, config_id: i32) -> Option<MoveSplineConfig> {
    if config_id <= 0 || info.get_key_int(&format!("scene_item_spline_opt:{config_id}:written")) != 1
    {
        return None;
    }
    let mut config = MoveSplineConfig::default();
    let mut has_config = false;

    let start = info.get_key_int(&format!("scene_item_spline_opt:{config_id}:start"));
    if start != ABSENT_VALUE {
        config.StartPoint = Some(MoveSplineConfigStartPointOneof::StartPointIndex(
            i32::try_from(start).ok()?,
        ));
        has_config = true;
    }
    let end = info.get_key_int(&format!("scene_item_spline_opt:{config_id}:end"));
    if end != ABSENT_VALUE {
        config.EndPoint = Some(MoveSplineConfigEndPointOneof::EndPointIndex(
            i32::try_from(end).ok()?,
        ));
        has_config = true;
    }
    let look_dir = info.get_key_int(&format!("scene_item_spline_opt:{config_id}:look_dir"));
    if look_dir != ABSENT_VALUE {
        config.LookDir = Some(MoveSplineConfigLookDirOneof::IsLookDir(look_dir != 0));
        has_config = true;
    }
    let cycle = info.get_key_int(&format!("scene_item_spline_opt:{config_id}:cycle"));
    if cycle != ABSENT_VALUE {
        config.Cycle = Some(MoveSplineConfigCycleOneof::CycleCount(
            i32::try_from(cycle).ok()?,
        ));
        has_config = true;
    }
    let circle = info.get_key_int(&format!("scene_item_spline_opt:{config_id}:circle"));
    if circle != ABSENT_VALUE {
        config.Circle = Some(MoveSplineConfigCircleOneof::IsCircle(circle != 0));
        has_config = true;
    }

    let _ = Assets::load_default;
    has_config.then_some(config)
}
