mod app_state;
use app_state::AppState;

use fsd28_lib::models::profile::Profile;
use fsd28_lib::create_profile;

use dialoguer::{theme::ColorfulTheme, Select, Input};



fn main() {
    let mut app_state = AppState::new();


    let selections = &["Load", "Create"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Load" => load_profile_dialog(&mut app_state),
        "Create" => create_profile_dialog(&mut app_state),
        _ => unreachable!(),
    }
}

fn load_profile_dialog(app_state: &mut AppState) {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the path to load the profile from")
        .interact_text()
        .unwrap();

    println!("Loading profile from: {}", path);
    // Implement the logic to load the profile using fsd28-lib
}

fn create_profile_dialog(app_state: &mut AppState) {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a name for the new profile")
        .interact_text()
        .unwrap();

    println!("Creating new profile.\n\n");
    app_state.add_profile(create_profile(name));

    println!("{}", app_state.get_selected().unwrap().display_ascii());

    // Implement the logic to create a new profile using fsd28-lib
}