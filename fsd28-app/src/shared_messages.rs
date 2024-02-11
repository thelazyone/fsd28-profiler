use fsd28_lib::Profile;

pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ViewRoster,
    ViewUnits,
    DeselectProfiles,
    Save,
    Load,

    _ToggleTheme, // Temporarly unused.

    // Loading
    FileContentReceived(String), // TODO should it be a &str?

    // Dumping profile updates
    UpdateProfiles(Vec<Profile>),
}