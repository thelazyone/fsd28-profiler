pub enum SharedMessage {
    NoOp, // Dummy message for no-operation
    
    ViewRoster,
    ViewUnits,
    Save,
    Load,

    ToggleTheme,
}