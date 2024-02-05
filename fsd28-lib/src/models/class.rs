use serde::{Deserialize, Serialize};
use super::damage_chart::DamageChart;
use super::characteristics::Characteristics;

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
}