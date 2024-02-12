use serde::{Deserialize, Serialize};


#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct DiceValue {
    shape : u32,
    number : u32,
    armor : u32
}

impl DiceValue {
    pub fn new(i_shape:u32, i_number:u32) -> Result<DiceValue, String>{
        if ![6,8,10,12].contains(&i_shape) {
            return Err("shape provided does not exist".to_string());
        }
        Ok(DiceValue{shape: i_shape, number: i_number, armor: 0})
    }

    pub fn new_from_string(i_string: String) ->  Result<DiceValue, String>{

        // There are two patterns in which the string could be formatted:
        let shape: u32;
        let number: u32;
        let armor: u32;
        
        // Armor pattern: "D10(2)" -> number = 1, shape = 10, armor = 2
        if let Some(start_parenthesis) = i_string.find('(') {
            if let Some(end_parenthesis) = i_string.find(')') {
                let shape_str = &i_string[1..start_parenthesis];
                let armor_str = &i_string[start_parenthesis + 1..end_parenthesis];
                shape = shape_str.parse::<u32>().map_err(|_| "Invalid shape".to_string())?;
                armor = armor_str.parse::<u32>().map_err(|_| "Invalid armor".to_string())?;
                if ![6,8,10,12].contains(&shape) {
                    return Err(format!("Shape provided does not exist: {}", i_string).to_string());
                }
                number = 1;
                Ok(DiceValue { shape, number, armor })
            }
            else {
                Err(format!("Badly formatted shape: {}", i_string).to_string())
            }
        } 
        
        // Damage pattern: "3d8" -> number = 3, shape = 8, armor = 0
        else {
            let parts: Vec<&str> = i_string.split('d').collect();
            (number, shape) = match parts.len() {
                1 => (1, parts[0].parse::<u32>().map_err(|_| "Invalid shape".to_string())?),
                2 => (parts[0].parse::<u32>().map_err(|_| "Invalid number".to_string())?, parts[1].parse::<u32>().map_err(|_| "Invalid shape".to_string())?),
                _ => return Err(format!("Shape provided does not exist: {}", i_string).to_string())
            };
            armor = 0;

            if ![6,8,10,12].contains(&shape) {
                return Err(format!("Shape provided does not exist: {}", i_string).to_string());
            }
            Ok(DiceValue { shape, number, armor })
        }

    }

    pub fn new_armor(i_shape:u32, i_armor:u32) -> Result<DiceValue, String>{
        if ![6,8,10,12].contains(&i_shape) {
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
        if ![-1, 1].contains(&delta) {
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
            0 => {
                if self.number == 0 {
                    return "-".to_string();
                }
                format!("{}d{}", self.number, self.shape)},
            _ => format!("d{}({})", self.shape, self.armor),
        }
    }
}

