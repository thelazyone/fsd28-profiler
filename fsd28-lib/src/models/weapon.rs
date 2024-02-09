use serde::{Serialize, Deserialize};
use super::action::Action; // Assuming Action is defined in action.rs

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponsConfig {
    pub weapons: Vec<Weapon>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub options: Vec<WeaponOption>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponOption {
    pub action: Action,
    pub is_base: bool,
}

impl Weapon {

    // Method to display the weapon and its actions, if needed
}

impl WeaponOption {

    // Method to display the category and its actions, if needed
}