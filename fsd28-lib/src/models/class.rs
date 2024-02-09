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
    pub special_abilities : Vec<String>,
    pub damage_profile: DamageChart,
    pub modifiers: Vec<Modifier>,
    pub cost: u32,
}

impl Class {

}