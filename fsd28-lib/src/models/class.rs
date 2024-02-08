use serde::{Deserialize, Serialize};
use super::damage_chart::DamageChart;
use super::characteristics::Characteristics;
use crate::models::modifier::Modifier;

#[derive(Clone, Serialize, Deserialize)]
pub struct ClassesConfig {
    pub classes: Vec<Class>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Tier {
    Goon, 
    Char,
    Hero
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub tier: Tier,
    pub characteristics: Characteristics,
    pub special_rules : String,
    pub damage_profile: DamageChart,
    pub modifiers: Vec<Modifier>,
}

impl Class {
    
    pub fn apply_modifier_effects(&mut self, modifier: &Modifier) {
        if let Some(effects_map) = modifier.effects.as_object() {
            for (key, value) in effects_map {
                match key.as_str() {

                    // TODO these are to be handled depending on the situation.
                    "defense" => {
                        if let Some(increase) = value.as_i64() {
                            self.characteristics.stat_def += increase as u32;
                        }
                    },
                    // "melee" => {
                    //     if let Some(new_value) = value.as_str() {
                    //         // Assume profile has a field for melee you want to change
                    //         self.characteristics.stat_melee = ;
                    //     }
                    // },
                    // Handle other keys (e.g., "abilities") similarly
                    _ => {}
                }
            }
        }
    }
}