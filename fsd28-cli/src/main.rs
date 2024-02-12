mod app_state;

use app_state::AppState;
use app_state::MenuStates;

use fsd28_lib::models::class::ClassesConfig;
use fsd28_lib::create_profile;
use fsd28_lib::get_classes;
use fsd28_lib::get_weapons;
use fsd28_lib::utils::pdf_ascii_generator::create_pdf_ascii;
use fsd28_lib::load_profiles;
use fsd28_lib::save_profiles;

use dialoguer::{theme::ColorfulTheme, Select, Input};



fn main() {
    let mut app_state = AppState::new();
    let mut menu_state = MenuStates::MainMenu;

    while menu_state != MenuStates::Exit {
        menu_state = match menu_state {
            MenuStates::MainMenu => main_menu_dialog(&mut app_state),
            MenuStates::CreateProfile => create_profile_dialog(&mut app_state),
            MenuStates::LoadProfiles => load_profile_dialog(&mut app_state),
            MenuStates::SaveProfiles => save_profile_dialog(&mut app_state),
            MenuStates::SelectProfile => select_profile_dialog(&mut app_state),
            MenuStates::EditProfile => edit_profile_dialog(&mut app_state),
            MenuStates::PrintAllProfiles => print_all_profiles_dialog(&mut app_state),
            MenuStates::AddAction => add_action_dialog(&mut app_state),
            //MenuStates::AddEquip => add_equip_action(&mut app_state,)
            MenuStates::EditName => edit_name_dialog(&mut app_state),
            // MenuStates::EditType => edit_profile(),
            // MenuStates::EditClass => edit_profile(),
            MenuStates::Exit => break,
        };
    }
}


fn main_menu_dialog(_: &mut AppState) -> MenuStates{
    let selections = &["Create", "Save", "Load", "Select", "Print", "Exit"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This is the FSD28 profile creator. What would you like to do?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Create" => MenuStates::CreateProfile,
        "Save" => MenuStates::SaveProfiles,
        "Load" => MenuStates::LoadProfiles,
        "Select" => MenuStates::SelectProfile,
        "Print" => MenuStates::PrintAllProfiles,
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
    let all_classes: ClassesConfig = get_classes("");
    let options: Vec<String> = all_classes
    .classes.iter()
    .map(|class| class.name.clone())
    .collect::<Vec<String>>();
    
    // Selecting the class here
    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select a class:")
    .default(0)
    .items(&options[..])
    .interact()
    .unwrap();
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
    print!("\x1B[2J");
    println!("Here is the selected profile:\n\n");
    println!("{}", app_state.get_selected().unwrap().display_ascii());
    let selections = &[
        // TODO add more
        "Change Name",
        "Add Action",
        "Return"]; 
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .default(1)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Change Name" => MenuStates::EditName,
        "Add Action" => MenuStates::AddAction,
        "Return" => MenuStates::MainMenu,
        "Load Profiles" => MenuStates::LoadProfiles,
        _ => unreachable!(),
    }
}

fn print_all_profiles_dialog(app_state: &mut AppState) -> MenuStates { 
    let selections = &["Yes", "Return"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("This will print ALL profiles created. Continue?")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    match selections[selection] {
        "Yes" => {
            create_pdf_ascii(app_state.get_all_profiles(), "out.pdf");
        },
        "Return" => (),
        _ => unreachable!(),
    };

    MenuStates::MainMenu
}

fn add_action_dialog(app_state: &mut AppState) -> MenuStates {

    // Now asking for the Action to add.
    // There are actions depending on the weapon.

    // BROKEN BY THE UPDATES - TODO TO FIX
        // Default actions are always there.
        let all_weapons = get_weapons("");

        // Filling the options
        let mut options: Vec<String> = all_weapons
        .weapons
        .iter()
        .map(|weapon| weapon.name.clone())
        .collect::<Vec<String>>();

        // Lastly adding the Cancel option.
        options.push("Cancel".to_string());
        
        // Selecting the class here
        let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an action to add:")
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();

        if selection == options.len() - 1 {
            // The last option (Return to Main Menu) was selected
            return MenuStates::EditProfile;
        } else {
            // TODO temp code to make it compile. 
            app_state.get_selected().unwrap().actions.push(all_weapons.weapons[selection].options[0].action.clone())
        }

    // After adding the class, back to edit class.
    MenuStates::EditProfile
}

fn edit_name_dialog(app_state: &mut AppState) -> MenuStates {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a new name for the new profile")
        .interact_text()
        .unwrap();

    app_state.get_selected().unwrap().name = name;

    MenuStates::EditProfile
}

fn load_profile_dialog(app_state: &mut AppState) -> MenuStates  {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the filename to load the profile from")
        .interact_text()
        .unwrap();
    let path = path + ".sav"; 

    println!("Loading profiles from: {}", path);
    match load_profiles(&path) {
        Ok(loaded_profiles)  => {
            app_state.clear_profiles();
            app_state.set_all_profiles(loaded_profiles);
            println!("{} profiles loaded.", app_state.get_all_profiles().len());
            MenuStates::MainMenu},
        Err(_) => {
            println!("Error loading profiles from file {}", path);
            MenuStates::MainMenu},
    }

}

fn save_profile_dialog(app_state: &mut AppState) -> MenuStates  {
    let path: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the filename to load the profile from")
        .interact_text()
        .unwrap();
    let path = path + ".sav"; 

    println!("Saving {} profiles to: {}", app_state.get_all_profiles().len(), path);
    match save_profiles(app_state.get_all_profiles().clone(), &path) {
        Ok(_) => (),
        Err(err) => println!("Error saving to file: {:?}", err),
    }

    MenuStates::MainMenu
}

