pub mod models;
pub mod utils;

// Expose key functions or structs if needed
pub use models::profile::Profile;

fn create_profile(name : String) -> Profile {
    Profile::new(name)
}