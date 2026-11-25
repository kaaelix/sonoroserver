use sonoro_gamesv_data::data_tables::create_buff_information;
use sonoro_proto::pb::{BuffEffectCd, FightBuffComponentPb, FightBuffInformation};

use crate::events::FsmBindBuffSource;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Shield {
    pub handle_id: i32,
    pub shield_id: i64,
    pub buff_handle_id: i32,
    pub current: i32,
    pub max: i32,
    pub priority: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AppliedBuffMod {
    pub attr_idx: i32,
    pub delta: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FsmBindBuffLease {
    pub source: FsmBindBuffSource,
    pub handle_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsmBindBuffRelease {
    Preserve,
    Remove(i32),
}

#[derive(Debug, Clone)]
pub struct FightBuffComponent {
    pub buffs: Vec<FightBuffInformation>,
    pub shields: Vec<Shield>,
    pub list_buff_effect_cd: Vec<BuffEffectCd>,
    pub client_born_buff_ids: Vec<i64>,
    pub client_born_message_id: i64,
    pub fsm_bind_sources: Vec<(i32, FsmBindBuffSource)>,
    pub fsm_bind_buff_leases: Vec<FsmBindBuffLease>,
    pub fsm_owned_buff_handles: Vec<i32>,
    pub prebaked_handles: Vec<i32>,
    pub applied_buff_mods: Vec<AppliedBuffMod>,
    pub next_handle_id: i32,
    pub next_shield_handle_id: i32,
}

impl Default for FightBuffComponent {
    fn default() -> Self {
        Self {
            buffs: Vec::new(),
            shields: Vec::new(),
            list_buff_effect_cd: Vec::new(),
            client_born_buff_ids: Vec::new(),
            client_born_message_id: -1,
            fsm_bind_sources: Vec::new(),
            fsm_bind_buff_leases: Vec::new(),
            fsm_owned_buff_handles: Vec::new(),
            prebaked_handles: Vec::new(),
            applied_buff_mods: Vec::new(),
            next_handle_id: 1,
            next_shield_handle_id: 1,
        }
    }
}

impl FightBuffComponent {
    pub fn alloc_handle_id(&mut self) -> i32 {
        let id = self.next_handle_id.max(1);
        self.next_handle_id = id.wrapping_add(1).max(1);
        id
    }

    pub fn alloc_shield_handle_id(&mut self) -> i32 {
        let id = self.next_shield_handle_id.max(1);
        self.next_shield_handle_id = id.wrapping_add(1).max(1);
        id
    }

    pub fn push_buff(
        &mut self,
        buff_id: i64,
        instigator_id: i64,
        entity_id: i64,
        is_active: bool,
        fsm_bind_source: Option<FsmBindBuffSource>,
    ) -> i32 {
        let handle = self.alloc_handle_id();
        self.buffs.push(create_buff_information(
            handle,
            buff_id,
            instigator_id,
            entity_id,
            is_active,
        ));
        if let Some(src) = fsm_bind_source {
            self.fsm_bind_sources.push((handle, src));
        }
        handle
    }

    pub fn find_by_handle(&self, handle_id: i32) -> Option<&FightBuffInformation> {
        self.buffs.iter().find(|b| b.HandleId == handle_id)
    }

    pub fn find_by_handle_mut(&mut self, handle_id: i32) -> Option<&mut FightBuffInformation> {
        self.buffs.iter_mut().find(|b| b.HandleId == handle_id)
    }

    pub fn get_by_buff_id(&self, buff_id: i64) -> Option<&FightBuffInformation> {
        self.buffs.iter().find(|b| b.BuffId == buff_id)
    }

    pub fn get_by_buff_id_and_instigator(
        &self,
        buff_id: i64,
        instigator_id: i64,
    ) -> Option<&FightBuffInformation> {
        self.buffs
            .iter()
            .find(|b| b.BuffId == buff_id && b.InstigatorId == instigator_id)
    }

    pub fn remove_by_handle(&mut self, handle_id: i32) -> Option<FightBuffInformation> {
        self.fsm_bind_sources.retain(|(h, _)| *h != handle_id);
        self.clear_fsm_bind_handle(handle_id);
        self.prebaked_handles.retain(|&h| h != handle_id);
        self.shields
            .retain(|s| s.buff_handle_id != handle_id && s.handle_id != handle_id);
        let pos = self.buffs.iter().position(|b| b.HandleId == handle_id)?;
        Some(self.buffs.remove(pos))
    }

    pub fn prepare_fsm_bind_acquire(&mut self, source: FsmBindBuffSource) -> bool {
        if let Some(lease) = self
            .fsm_bind_buff_leases
            .iter()
            .find(|l| l.source == source)
            .copied()
        {
            if self.find_by_handle(lease.handle_id).is_some() {
                return false;
            }
            self.clear_fsm_bind_handle(lease.handle_id);
        }
        true
    }

    pub fn add_fsm_bind_lease(
        &mut self,
        source: FsmBindBuffSource,
        handle_id: i32,
        owns_handle: bool,
    ) {
        if owns_handle && !self.fsm_owned_buff_handles.contains(&handle_id) {
            self.fsm_owned_buff_handles.push(handle_id);
        }
        self.fsm_bind_buff_leases
            .push(FsmBindBuffLease { source, handle_id });
    }

    pub fn mark_fsm_bind_handle_external(&mut self, handle_id: i32) {
        self.fsm_owned_buff_handles.retain(|&h| h != handle_id);
    }

    pub fn release_fsm_bind_lease(
        &mut self,
        source: FsmBindBuffSource,
    ) -> Option<FsmBindBuffRelease> {
        let lease_idx = self
            .fsm_bind_buff_leases
            .iter()
            .position(|l| l.source == source)?;
        let handle_id = self.fsm_bind_buff_leases.swap_remove(lease_idx).handle_id;
        if self
            .fsm_bind_buff_leases
            .iter()
            .any(|l| l.handle_id == handle_id)
        {
            return Some(FsmBindBuffRelease::Preserve);
        }
        if !self.fsm_owned_buff_handles.contains(&handle_id) {
            return Some(FsmBindBuffRelease::Preserve);
        }
        self.fsm_owned_buff_handles.retain(|&h| h != handle_id);
        Some(FsmBindBuffRelease::Remove(handle_id))
    }

    fn clear_fsm_bind_handle(&mut self, handle_id: i32) {
        self.fsm_bind_buff_leases
            .retain(|l| l.handle_id != handle_id);
        self.fsm_owned_buff_handles.retain(|&h| h != handle_id);
    }

    pub fn mark_prebaked_handle(&mut self, handle_id: i32) {
        if !self.prebaked_handles.contains(&handle_id) {
            self.prebaked_handles.push(handle_id);
        }
    }

    pub fn is_prebaked_handle(&self, handle_id: i32) -> bool {
        self.prebaked_handles.contains(&handle_id)
    }

    pub fn set_applied_buff_mods(&mut self, mods: Vec<AppliedBuffMod>) {
        self.applied_buff_mods = mods;
    }

    pub fn upsert_shield(&mut self, shield: Shield) {
        if let Some(existing) = self
            .shields
            .iter_mut()
            .find(|s| s.handle_id == shield.handle_id)
        {
            *existing = shield;
        } else {
            self.shields.push(shield);
        }
    }

    pub fn remove_shield_by_handle(&mut self, handle_id: i32) {
        self.shields
            .retain(|s| s.handle_id != handle_id && s.buff_handle_id != handle_id);
    }

    pub fn total_shield(&self) -> i32 {
        self.shields
            .iter()
            .filter(|s| s.current > 0)
            .fold(0i32, |acc, s| acc.saturating_add(s.current))
    }

    pub fn fsm_bind_source(&self, handle_id: i32) -> Option<FsmBindBuffSource> {
        self.fsm_bind_sources
            .iter()
            .find(|(h, _)| *h == handle_id)
            .map(|(_, s)| *s)
    }

    pub fn stack_count_of(&self, buff_id: i64) -> i32 {
        self.buffs
            .iter()
            .filter(|b| b.BuffId == buff_id)
            .map(|b| b.StackCount)
            .sum()
    }

    pub fn to_proto(&self) -> FightBuffComponentPb {
        FightBuffComponentPb {
            FightBuffInfos: self.buffs.clone(),
            ListBuffEffectCd: self.list_buff_effect_cd.clone(),
            ClientBornBuffIds: self.client_born_buff_ids.clone(),
            ClientBornMessageId: self.client_born_message_id,
        }
    }
}
