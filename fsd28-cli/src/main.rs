mod app_state;
use app_state::AppState;
use app_state::MenuStates;

use fsd28_lib::models::class::ClassesConfig;
use fsd28_lib::create_profile;
use fsd28_lib::get_classes;

use dialoguer::{theme::ColorfulTheme, Select, Input};



fn main() {
    let mut app_state = AppState::new();
    let mut menu_state = MenuStates::MainMenu;

    while menu_state != MenuStates::Exit {
        menu_state = match menu_state {
            MenuStates::MainMenu => main_menu_dialog(&mut app_state),
            MenuStates::CreateProfile => create_profile_dialog(&mut app_state),
            //MenuStates::LoadProfiles => load_profile_dialog(&mut app_state),
            MenuStates::SelectProfile => select_profile_dialog(&mut app_state),
            MenuStates::EditProfile => edit_profile_dialog(&mut app_state),
            MenuStates::EditName => edit_name_dialog(&mut app_state),
            // MenuStates::EditType => edit_profile(),
            // MenuStates::EditClass => edit_profile(),
            MenuStates::Exit => break,
            _ => {println!("State not covered yet!"); MenuStates::MainMenu},
        };
    }
}


fn main_menu_dialog(app_state: &mut AppState) -> MenuStates{
    let selections = &["Create", "Select", "Exit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Create" => MenuStates::CreateProfile,
        "Select" => MenuStates::SelectProfile,
        "Exit" => MenuStates::Exit,
        _ => unreachable!(),
    }
}

fn create_profile_dialog(app_state: &mut AppState) -> MenuStates {

    // Asking for the name
    let selected_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a name for the new profile")
        .interact_text()
        .unwrap();

    // Now asking for the class
    let all_classes = get_classes("");
    let mut options: Vec<String> = all_classes
    .classes.iter()
    .map(|class| class.name.clone())
    .collect::<Vec<String>>();
    
    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select a class:")
    .default(0)
    .items(&options[..])
    .interact()
    .unwrap();

    //Selection is the index. 
    let selected_class = all_classes.classes[selection].clone();

    app_state.add_profile(create_profile(
        selected_name, 
        selected_class));

    MenuStates::EditProfile
}

fn select_profile_dialog(app_state: &mut AppState) -> MenuStates {

    let mut options: Vec<String> = app_state
        .get_all_profiles()
        .iter()
        .map(|profile| profile.name.clone())
        .collect::<Vec<String>>();

    // Add an option to return to the main menu
    options.push("Return to Main Menu".to_string());

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a profile or return to the main menu")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

    if selection == options.len() - 1 {
        // The last option (Return to Main Menu) was selected
        MenuStates::MainMenu
    } else {
        // Update the selected index in AppState
        app_state.set_selected(selection);
        MenuStates::EditProfile // or any other state you want to transition to
    }
}

fn edit_profile_dialog(app_state: &mut AppState) -> MenuStates {

    println!("Here is the selected profile:");
    println!("{}", app_state.get_selected().unwrap().display_ascii());
    let selections = &[
        // TODO add more
        "Change Name",
        "Return"]; 
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Change Name" => MenuStates::EditName,
        "Return" => MenuStates::MainMenu,
        "Load Profiles" => MenuStates::LoadProfiles,
        _ => unreachable!(),
    }
}

fn edit_name_dialog(app_state: &mut AppState) -> MenuStates {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a new name for the new profile")
        .interact_text()
        .unwrap();

    app_state.get_selected().unwrap().name = name;

    MenuStates::EditProfile
}

fn load_profile_dialog(app_state: &mut AppState) {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the path to load the profile from")
        .interact_text()
        .unwrap();

    println!("Loading profile from: {}", path);
    // Implement the logic to load the profile using fsd28-lib
}

