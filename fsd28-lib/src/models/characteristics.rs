use serde::{Deserialize, Serialize};
use colored::Colorize;

use crate::models::dice_value::DiceValue;


#[derive(Clone, Serialize, Deserialize)]
pub struct Characteristics {
    pub stat_cmd: u32,
    pub stat_def: u32,
    pub stat_save: DiceValue,
    pub stat_move: u32,
    pub stat_shoot: DiceValue,
    pub stat_melee: DiceValue,
}

impl Characteristics {
    pub fn new_default() -> Characteristics{
        Characteristics {
            stat_cmd: 0,
            stat_def: 4,
            stat_save: DiceValue::new_armor(8, 2).unwrap(),
            stat_move: 3,
            stat_shoot: DiceValue::new(6, 2).unwrap(),
            stat_melee: DiceValue::new(6, 2).unwrap(),
        }
    }

    pub fn display_ascii(&self) -> String {
        let mut out_string = String::new();
        out_string += "\nCmd   = ";
        out_string += &*self.stat_cmd.to_string().bold().blue().to_string();
        out_string += "\tDef   = ";
        out_string += &*format!("{}+", self.stat_def).bold().blue().to_string();
        out_string += "\tSave  = ";
        out_string += &*self.stat_save.display().bold().blue().to_string();
        out_string += "\nMove  = ";
        out_string += &*self.stat_move.to_string().bold().blue().to_string();
        out_string += "\tShoot = ";
        out_string += &*self.stat_shoot.display().bold().blue().to_string();
        out_string += "\tMelee = ";
        out_string += &*self.stat_melee.display().bold().blue().to_string();
        out_string
    }
}
