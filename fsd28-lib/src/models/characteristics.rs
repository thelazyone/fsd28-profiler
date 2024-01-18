use serde::{Deserialize, Serialize};

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
}
