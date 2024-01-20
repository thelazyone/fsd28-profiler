pub mod models;
pub mod utils;

use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

// Expose key functions or structs if needed
pub use models::profile::Profile;
pub use models::class::ClassesConfig;
pub use models::class::Class;
pub use models::action::ActionsConfig;

// GAME DATA LOADING

pub fn get_classes(i_path : &str) -> ClassesConfig {
    let mut path = i_path;
    if i_path.is_empty() {
        path = "./fsd28-lib/data/classes.json";
    }
    let file_content = read_to_string(path).expect("Failed to read file");
    serde_json::from_str(&file_content).unwrap()
}

pub fn get_default_actions(i_path: &str) -> ActionsConfig {
    let mut path = i_path;
    if i_path.is_empty() {
        path = "./fsd28-lib/data/default_actions.json";
    }
    let file_content = read_to_string(path).expect("Failed to read file");
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
