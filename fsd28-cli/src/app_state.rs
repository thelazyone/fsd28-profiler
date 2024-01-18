use fsd28_lib::models::profile::Profile;

pub struct AppState {
    profiles: Vec<Profile>,
    selected: Option<usize>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            profiles: Vec::new(),
            selected: None,
        }
    }

    pub fn add_profile(&mut self, new_profile: Profile) {
        
        // Adding in the new profile and then setting the index to current
        self.profiles.push(new_profile);
        self.selected = Some(self.profiles.len() - 1);

    }

    pub fn get_selected(&mut self) -> Option<&mut Profile> {
        if let Some(index) = self.selected {

            if index >= self.profiles.len() {
                println!("Error: profile index out of bounds.")
            }

            return self.profiles.get_mut(index);
        }
        else {
            println!("Error: no state selected."); 
            return None;
        }
    }
}