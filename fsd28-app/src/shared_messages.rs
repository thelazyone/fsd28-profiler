pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ViewRoster,
    ViewUnits,
    Save,
    Load,

    ToggleTheme, // Temporarly unused.

    // Loading
    FileContentReceived(String), // TODO should it be a &str?
}