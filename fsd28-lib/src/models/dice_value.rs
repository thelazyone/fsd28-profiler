use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct DiceValue {
    shape : u32,
    number : u32,
    armor : u32
}

impl DiceValue {
    pub fn new(i_shape:u32, i_number:u32) -> Result<DiceValue, String>{
        if !vec![6,8,10,12].contains(&i_shape) {
            return Err("shape provided does not exist".to_string());
        }
        Ok(DiceValue{shape: i_shape, number: i_number, armor: 0})
    }

    pub fn new_armor(i_shape:u32, i_armor:u32) -> Result<DiceValue, String>{
        if !vec![6,8,10,12].contains(&i_shape) {
            return Err("shape provided does not exist".to_string());
        }
        Ok(DiceValue{shape: i_shape, number: 0, armor: i_armor})
    }

    pub fn change_number(self, delta:i32) -> Result<DiceValue, String>{
        let temp = self.number as i32 + delta;
        if temp < 1 {
            return Err("there must be always at least one die".to_string())
        }
        Ok(DiceValue{shape: self.shape, number: temp as u32, armor: self.armor})
    }

    pub fn change_category(self, delta:i32) -> Result<DiceValue, String>{
        if !vec![-1, 1].contains(&delta) {
            return Err("change can be only by one category".to_string())
        }
        Ok(DiceValue{shape: (self.shape as i32 + delta * 2) as u32, number: self.number, armor: self.armor})
    }

    pub fn change_armor(self, delta:i32) -> Result<DiceValue, String>{
        let temp = self.armor as i32 + delta;
        if temp < 1 {
            return Err("cannot bring armor below 1".to_string())
        }
        Ok(DiceValue{shape: self.shape, number: self.number, armor: temp as u32})
    }

    pub fn display(&self) -> String {
        match self.armor {
            0 => format!("{}d{}", self.number, self.shape),
            _ => format!("d{}({})", self.shape, self.armor),
        }
    }
}

