use crate::models::dice_value::DiceValue;
use crate::models::damage_chart::DamageChart;
use crate::models::action::Action;
use crate::models::characteristics::Characteristics;
use crate::models::class::Class;

use colored::Colorize;

use super::class;

#[derive(Clone)]
pub struct Profile {
    pub name: String,
    description: String,
    characteristics: Characteristics,
    special_abilities: String,
    damage_chart: DamageChart, // Temporary
    actions: Vec<Action>,
    cost: u32,
}

impl Profile {

    pub fn new(i_name : String, i_class : Class) -> Profile {
        Profile {
            name : i_name,
            description : i_class.name,
            characteristics : i_class.characteristics,
            special_abilities: "none".to_string(),
            damage_chart: i_class.damage_profile,
            actions: Vec::<Action>::new(),
            cost: 42,
        }
    } 

    pub fn display_ascii (&self) -> String {
        let mut out_string = String::new();
        out_string += "Character Name:     ";
        out_string += &*self.name.bold().blue().to_string();
        out_string += "\nDescription:        ";
        out_string += &*self.description.bold().blue().to_string();
        out_string += &*self.characteristics.display_ascii();
        out_string += "\n\nSpecial Abilities:  ";
        out_string += &*self.special_abilities.bold().blue().to_string();
        out_string += "\n\n";
        out_string += &*self.damage_chart.display_ascii();

        out_string
    }
}


