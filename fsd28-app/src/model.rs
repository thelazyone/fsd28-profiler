//use fsd28_lib::models::profile::Profile;


use serde::{Deserialize, Serialize};

// The App Model contains 
// - A list of profiles that have been created
// - A roster of selected profiles (in the future it will be a list of rosters)
#[derive(Clone, Deserialize, Serialize)]
pub struct Model {
    //profiles: Vec<Profile>,
    roster: Vec<String>,
}

impl Model {
    pub fn new() -> Model {
        Model { 
            //profiles: Vec::<Profile>::new(),
            roster : Vec::<String>::new(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}