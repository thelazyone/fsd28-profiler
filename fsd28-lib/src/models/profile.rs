use crate::models::dice_value::DiceValue;
use crate::models::damage_chart::DamageChart;
use crate::models::action::Action;

use colored::Colorize;

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

    pub fn new(i_name : String) -> Profile {
        Profile {
            name : i_name,
            description : "temp description".to_string(),
            characteristics : Characteristics::new_default(),
            special_abilities: "none".to_string(),
            damage_chart: DamageChart::new_default(),
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
        out_string += "\n";
        out_string += "\nCmd   = ";
        out_string += &*self.characteristics.stat_cmd.to_string().bold().blue().to_string();
        out_string += "\tDef   = ";
        out_string += &*format!("{}+", self.characteristics.stat_def).bold().blue().to_string();
        out_string += "\tSave  = ";
        out_string += &*self.characteristics.stat_save.display().bold().blue().to_string();
        out_string += "\nMove  = ";
        out_string += &*self.characteristics.stat_move.to_string().bold().blue().to_string();
        out_string += "\tShoot = ";
        out_string += &*self.characteristics.stat_shoot.display().bold().blue().to_string();
        out_string += "\tMelee = ";
        out_string += &*self.characteristics.stat_melee.display().bold().blue().to_string();
        out_string += "\n\nSpecial Abilities:  ";
        out_string += &*self.special_abilities.bold().blue().to_string();
        out_string += "\n\n";
        out_string += &*self.damage_chart.display_ascii();

        out_string
    }
}


#[derive(Clone)]
struct Characteristics {
    pub stat_cmd: u32,
    pub stat_def: u32,
    pub stat_save: DiceValue,
    pub stat_move: u32,
    pub stat_shoot: DiceValue,
    pub stat_melee: DiceValue,
}

impl Characteristics {
    fn new_default() -> Characteristics{
        Characteristics {
            stat_cmd: 0,
            stat_def: 4,
            stat_save: DiceValue::new_armor(8, 2).unwrap(),
            stat_move: 3,
            stat_shoot: DiceValue::new(6, 2).unwrap(),
            stat_melee: DiceValue::new(6, 2).unwrap(),
        }
    }
}
