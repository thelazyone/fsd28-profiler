enum Color {
    Red,
    Yellow,
    Green
}

pub struct DamageChart {
    intervals: Vec<(u32, Color, String)>
}

impl DamageChart {

    pub fn new_default() -> DamageChart {
        DamageChart {intervals: vec![(1, Color::Red, "DEAD".to_string()), (2, Color::Yellow, "MOV".to_string()), (2, Color::Yellow, "ARM".to_string()), (2, Color::Green, "PIN".to_string()),]}
    }

    // TODO implement proper new

    // TODO implement display_ascii

    // TODO implement display_png
}

