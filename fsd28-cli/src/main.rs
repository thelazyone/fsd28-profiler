use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
    let selections = &["Load", "Create"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Load" => {
            println!("Loading profile...");
            // Implement loading logic
        }
        "Create" => {
            println!("Creating new profile...");
            // Implement creation logic
        }
        _ => unreachable!(), // Since selections are predefined, this should never happen
    }
}
