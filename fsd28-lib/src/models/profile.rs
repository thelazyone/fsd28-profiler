use crate::models::damage_chart::DamageChart;
use crate::models::action::Action;
use crate::models::characteristics::Characteristics;
use crate::models::class::Class;
use crate::models::class::Tier;
use crate::models::modifier::Modifier;

use serde::{Deserialize, Serialize};

// For ascii display
use colored::Colorize;


#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub class_name: String,
    pub selected_modifiers: Vec<Modifier>,
    pub description: String,
    pub tier: Tier,
    pub characteristics: Characteristics,
    pub special_abilities: Vec<String>,
    pub damage_chart: DamageChart, // Temporary
    pub actions: Vec<Action>,
    pub cost: u32,
}


fn apply_modifier_effects(profile: &mut Profile, modifier: &Modifier) {
    if let Some(effects_map) = modifier.effects.as_object() {

        // Searching for the keyword of the modifier
        for (key, value) in effects_map {
            match key.as_str() {

                // TODO these are to be handled depending on the situation.
                "ability" => {
                    if let Some(new_ability) = value.as_str() {
                        profile.special_abilities.push(new_ability.to_string());
                    }
                }
                _ => {}
            }
        }

        // Adding the points to the cost of the unit.
        profile.cost += modifier.points;
    }
}

impl Profile {

    pub fn new(i_name : String, i_class : Class) -> Profile {
        Profile {
            name : i_name,
            description : i_class.name.clone(),
            class_name : i_class.name,
            selected_modifiers : Vec::<Modifier>::new(),
            tier: i_class.tier,
            characteristics : i_class.characteristics,
            special_abilities: Vec::<String>::new(),
            damage_chart: i_class.damage_profile,
            actions: Vec::<Action>::new(),
            cost: i_class.cost,
        }
    } 

    pub fn get_final_profile(&self) -> Profile {
        let mut modified_profile = self.clone();

        // Applying the modifiers
        for modifier in &self.selected_modifiers{
            apply_modifier_effects( &mut modified_profile, modifier);
        }

        // Calculating the actions costs
        for action in &self.actions{
            modified_profile.cost += action.points;
        }

        // Updating the profile description.
        if !modified_profile.selected_modifiers.is_empty() {
            modified_profile.description += " (";
            modified_profile.description += &(modified_profile.selected_modifiers.iter().map(|modifier| {modifier.id.clone()}).collect::<Vec<String>>().join(", "));
            modified_profile.description += ")";
        }

        modified_profile
    }

    pub fn display_ascii (&self) -> String {
        let mut out_string = String::new();
        // Infos
        out_string += "Character Name:     ";
        out_string += &*self.name.bold().blue().to_string();
        out_string += "\nDescription:        ";
        out_string += &*self.description.bold().blue().to_string();
        out_string += "\nCost:               ";
        out_string += &*self.cost.to_string().bold().blue().to_string();

        // Characteristics
        out_string += "\n\n";
        out_string += &*self.characteristics.display_ascii();
        out_string += "\n\nActions:            ";
        for action in &self.actions {
            out_string += "\n";
            out_string += &*action.display_ascii(&self.tier);
        }

        // Abilities and damage chart
        out_string += "\n\nSpecial Abilities:  ";
        out_string += &*self.special_abilities.join(", ").bold().blue().to_string();
        out_string += "\n\n";
        out_string += &*self.damage_chart.display_ascii();

        out_string
    }
}


