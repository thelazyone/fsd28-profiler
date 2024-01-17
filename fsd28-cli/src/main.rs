use fsd28_lib::models::profile::Profile;
use fsd28_lib::create_profile;

use dialoguer::{theme::ColorfulTheme, Select, Input};



fn main() {
    let selections = &["Load", "Create"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Load" => load_profile_dialog(),
        "Create" => create_profile_dialog(),
        _ => unreachable!(),
    }
}

fn load_profile_dialog() {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the path to load the profile from")
        .interact_text()
        .unwrap();

    println!("Loading profile from: {}", path);
    // Implement the logic to load the profile using fsd28-lib
}

fn create_profile_dialog() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a name for the new profile")
        .interact_text()
        .unwrap();

    println!("Creating new profile.\n\n");
    let mut profile = create_profile(name);
    println!("{}", profile.display_ascii());

    // Implement the logic to create a new profile using fsd28-lib
}