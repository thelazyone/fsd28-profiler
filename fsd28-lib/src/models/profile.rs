use crate::models::dice_value::DiceValue;
use crate::models::damage_chart::DamageChart;
use crate::models::action::Action;

pub struct Profile {
    name: String,
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

    pub fn display_ascii (self) -> String {
        format!(
            "{}\n{}\nCmd = {}\n", 
            self.name, 
            self.description, 
            self.characteristics.stat_cmd
        ).to_string()
    }
}


struct Characteristics {
    stat_cmd: u32,
    stat_def: u32,
    stat_save: DiceValue,
    stat_move: u32,
    stat_shoot: DiceValue,
    stat_melee: DiceValue,
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
