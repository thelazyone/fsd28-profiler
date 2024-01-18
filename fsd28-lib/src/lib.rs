pub mod models;
pub mod utils;

use std::fs;


// Expose key functions or structs if needed
pub use models::profile::Profile;
pub use models::class::ClassesConfig;
pub use models::class::Class;

pub fn get_classes(i_path : &str) -> ClassesConfig {
    let file_content = fs::read_to_string(i_path).expect("Failed to read file");
    serde_json::from_str(&file_content).unwrap()
}

pub fn create_profile(i_name : String, i_class : Class) -> Profile {
    Profile::new(i_name, i_class)
}
