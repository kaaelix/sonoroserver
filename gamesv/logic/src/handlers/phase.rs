use std::io;

use sonoro_proto::pb::{PushContextIdNotify, PushDataCompleteNotify};

use crate::event_queue::EventQueue;
use crate::events::{Event, PendingFlow};
use crate::helpers::quest_engine;
use crate::helpers::quest_types::QuestContext;
use crate::sink::PushSink;

#[allow(clippy::too_many_arguments)]
pub fn run_enter_game_phase(
    events: &mut EventQueue,
    player_id: i32,
    map_config_id: i32,
    instance_id: i32,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
    keep_team: bool,
    pending_flow: Option<PendingFlow>,
) {
    let has_pos =
        x.is_finite() && y.is_finite() && z.is_finite() && (x != 0.0 || y != 0.0 || z != 0.0);
    let has_yaw = yaw.is_finite() && yaw != 0.0;
    let _ = keep_team;
    events.enqueue(Event::PushData {
        player_id,
        map_config_id,
        instance_id,
    });
    events.enqueue(Event::PushDataComplete { player_id });
    events.enqueue(Event::InitialSceneJoin {
        player_id,
        map_config_id,
        instance_id,
        location: has_pos.then_some([x, y, z]),
        rotation: has_yaw.then_some([0.0, 0.0, yaw]),
        pending_flow,
    });
}

pub fn notify_push_data_complete<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
) -> io::Result<()> {
    // Push quests after all push_data handlers (including FuncOpenNotify) so quest UI systems are unlocked.
    quest_engine::sync_on_login(ctx)?;
    ctx.sink
        .push(PushContextIdNotify::MSG_ID, &PushContextIdNotify { Id: 0 });
    ctx.sink
        .push(PushDataCompleteNotify::MSG_ID, &PushDataCompleteNotify {});
    Ok(())
}

pub fn on_quest_timers_due<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    now_ms: i64,
) -> io::Result<()> {
    quest_engine::tree::on_timers_due(ctx, now_ms)
}

pub fn on_level_play_timers_due<S: PushSink + ?Sized>(
    ctx: &mut QuestContext<'_, S>,
    now_ms: i64,
) -> io::Result<()> {
    while let Some(due_node) = ctx
        .scene
        .as_deref_mut()
        .and_then(|s| s.lp_timers.pop_due(now_ms))
    {
        if let Some(mut lp_ctx) = crate::helpers::level_play::LevelPlayContext::from_quest_ctx(ctx)
        {
            let _ = crate::helpers::level_play::on_timer_due(
                &mut lp_ctx,
                due_node.quest_id,
                due_node.node_id,
            )?;
        }
    }
    while let Some(due_node) = ctx
        .scene
        .as_deref_mut()
        .and_then(|s| s.lp_fail_timers.pop_due(now_ms))
    {
        if let Some(mut lp_ctx) = crate::helpers::level_play::LevelPlayContext::from_quest_ctx(ctx)
        {
            let _ = crate::helpers::level_play::on_fail_timer_due(
                &mut lp_ctx,
                due_node.quest_id,
                due_node.node_id,
            )?;
        }
    }
    Ok(())
}
