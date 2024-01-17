use dialoguer::{theme::ColorfulTheme, Select, Input};



fn main() {
    let selections = &["Load", "Create"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Load" => load_profile(),
        "Create" => create_profile(),
        _ => unreachable!(),
    }
}

fn load_profile() {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the path to load the profile from")
        .interact_text()
        .unwrap();

    println!("Loading profile from: {}", path);
    // Implement the logic to load the profile using fsd28-lib
}

fn create_profile() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a name for the new profile")
        .interact_text()
        .unwrap();

    println!("Creating new profile.");

    // Implement the logic to create a new profile using fsd28-lib
}