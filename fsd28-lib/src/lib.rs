pub mod models;
pub mod utils;

use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

// Hardcoding the classes for web applications or general default info
const CLASSES_JSON: &str = include_str!("../data/classes.json");
const WEAPONS_JSON: &str = include_str!("../data/weapons.json");


// Expose key functions or structs if needed
pub use models::profile::Profile;
pub use models::class::ClassesConfig;
pub use models::class::Class;
pub use models::weapon::WeaponsConfig;
pub use models::weapon::Weapon; // TODO this might be better incapsulated?
pub use models::weapon::WeaponOption; // TODO this might be better incapsulated?
pub use models::action::Action; // TODO this might be better incapsulated?
pub use models::modifier::Modifier; // TODO this might be better incapsulated?

// GAME DATA LOADING
pub fn get_classes(i_path : &str) -> ClassesConfig {
    let file_content: String = 
        if i_path.is_empty() {
            CLASSES_JSON.to_string()
        }
        else {
            read_to_string(i_path).expect("Failed to read file")
        };

    serde_json::from_str(&file_content).unwrap()
}

pub fn get_weapons(i_path: &str) -> WeaponsConfig {
    let file_content: String = 
        if i_path.is_empty() {
            WEAPONS_JSON.to_string()
        }
        else {
            read_to_string(i_path).expect("Failed to read file")
        };
    serde_json::from_str(&file_content).unwrap()
}


// PROFILES HANDLING

pub fn create_profile(i_name : String, i_class : Class) -> Profile {
    Profile::new(i_name, i_class)
}

pub fn save_profiles(i_profiles: Vec<Profile>, i_path : &str) -> Result<(), std::io::Error>{
    let out_content = serde_json::to_string(&i_profiles).unwrap();
    let mut file = match File::create(i_path) {
        Ok(content) => content,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't create file!")),
    };
    file.write_all(out_content.as_bytes())
}

pub fn load_profiles(i_path : &str) -> Result<Vec<Profile>, std::io::Error> {
    let file_content = match read_to_string(i_path) {
        Ok(content) => content,
        Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't read file!")),
    };

    match serde_json::from_str(&file_content) {
        Ok(profiles) => Ok(profiles),
        Err(_) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Failed to parse JSON")),
    }
}
