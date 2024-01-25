//use fsd28_lib::models::profile::Profile;


use serde::{Deserialize, Serialize};

use fsd28_lib::models::class::ClassesConfig;
use fsd28_lib::create_profile;
use fsd28_lib::get_classes;
use fsd28_lib::get_default_actions;
use fsd28_lib::utils::pdf_ascii_generator::create_pdf_ascii;
use fsd28_lib::load_profiles;
use fsd28_lib::save_profiles;
use fsd28_lib::models::profile::Profile;

// For browser debugging
use web_sys::{console, HtmlAnchorElement};


// The App Model contains 
// - A list of profiles that have been created
// - A roster of selected profiles (in the future it will be a list of rosters)
#[derive(Clone, Deserialize, Serialize)]
pub struct Model {
    pub profiles: Vec<Profile>,
    pub roster: Vec<String>,
}

impl Model {
    pub fn new() -> Model {
        Model { 
            profiles: Vec::<Profile>::new(),
            roster : Vec::<String>::new(),
        }
    }

    // JSON serialization (static methods):
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {

        let model: Model = serde_json::from_str(json_str)?;

        // TODO version is not implemented yet.
        // if model.version < 1 { // Assuming 1 is the current version // TODO handle versioning better
        //     // Handle older versions differently
        //     // For now, just return an error
        //     return Err(de::Error::custom("Roster version is too old"));
        // }

        console::log_1(&format!("Loaded model with {} profiles and {} rosters", model.profiles.len(), model.roster.len()).into());

        Ok(model)
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}