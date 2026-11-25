use sonoro_proto::enums::EAttributeType;
use sonoro_proto::pb::{AttrData, AttributeComponentPb, GameplayAttributeData};
use sonoro_proto::wire::PbEnum;

pub const MAX_ATTR_LEN: usize = 144;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attribute {
    pub base: i32,
    pub increment: i32,
    pub current: i32,
}

#[derive(Debug, Clone)]
pub struct AttributeComponent {
    pub attributes: Vec<Attribute>,
    pub hardness_mode_id: i32,
    pub rage_mode_id: i32,
}

impl Default for AttributeComponent {
    fn default() -> Self {
        Self {
            attributes: vec![Attribute::default(); MAX_ATTR_LEN],
            hardness_mode_id: 0,
            rage_mode_id: 0,
        }
    }
}

impl AttributeComponent {
    pub fn create(props: &[i32]) -> Self {
        Self::create_with_modes(props, 0, 0)
    }

    pub fn create_with_modes(props: &[i32], hardness_mode_id: i32, rage_mode_id: i32) -> Self {
        let mut attributes = vec![Attribute::default(); MAX_ATTR_LEN];
        for (idx, &val) in props.iter().enumerate() {
            if idx >= attributes.len() {
                break;
            }
            attributes[idx] = Attribute {
                base: val,
                increment: 0,
                current: val,
            };
        }
        Self {
            attributes,
            hardness_mode_id,
            rage_mode_id,
        }
    }

    pub fn create_with_increments(base_props: &[i32], add_props: &[i32]) -> Self {
        let mut attributes = vec![Attribute::default(); MAX_ATTR_LEN];
        for (idx, attr) in attributes.iter_mut().enumerate() {
            let base = base_props.get(idx).copied().unwrap_or(0);
            let increment = add_props.get(idx).copied().unwrap_or(0);
            *attr = Attribute {
                base,
                increment,
                current: base + increment,
            };
        }
        Self {
            attributes,
            hardness_mode_id: 0,
            rage_mode_id: 0,
        }
    }

    pub fn set_prop(&mut self, attr_type: EAttributeType, value: i32) {
        let idx = attr_type.to_i32() as usize;
        if idx < self.attributes.len() {
            self.attributes[idx] = Attribute {
                base: value,
                increment: 0,
                current: value,
            };
        }
    }

    pub fn get_current(&self, attr_type: EAttributeType) -> i32 {
        let idx = attr_type.to_i32() as usize;
        self.attributes.get(idx).map_or(0, |a| a.current)
    }

    pub fn to_proto(&self) -> AttributeComponentPb {
        let mut attr_data = Vec::new();
        let max_enum = EAttributeType::MAX.to_i32();
        for (idx, attr) in self.attributes.iter().enumerate() {
            let idx_i32 = idx as i32;
            if idx_i32 >= max_enum {
                break;
            }
            attr_data.push(AttrData {
                AttributeType: Some(EAttributeType::from_i32_or_default(idx_i32)),
                WhiteValue: attr.base,
                ValueIncrement: attr.increment,
                CurrentValue: attr.current,
            });
        }
        AttributeComponentPb {
            HardnessModeId: self.hardness_mode_id,
            RageModeId: self.rage_mode_id,
            AttrData: attr_data,
        }
    }

    pub fn gameplay_attribute_list(&self) -> Vec<GameplayAttributeData> {
        let max_enum = EAttributeType::MAX.to_i32();
        let mut out = Vec::new();
        for (idx, attr) in self.attributes.iter().enumerate() {
            let idx_i32 = idx as i32;
            if idx_i32 <= 0 || idx_i32 >= max_enum {
                continue;
            }
            let attr_type = EAttributeType::from_i32_or_default(idx_i32);
            let is_ratio = matches!(
                attr_type,
                EAttributeType::Atk | EAttributeType::LifeMax | EAttributeType::Def
            );
            out.push(GameplayAttributeData {
                AttributeType: Some(attr_type),
                BaseValue: if is_ratio { attr.base } else { attr.current },
                CurrentValue: attr.current,
            });
        }
        out
    }
}
