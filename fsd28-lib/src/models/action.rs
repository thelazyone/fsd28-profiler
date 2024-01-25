use serde::{Serialize, Deserialize};
use colored::Colorize;

#[derive(Clone, Serialize, Deserialize)]
pub struct ActionsConfig {
    pub actions: Vec<Action>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub cost: Vec<(u32, u32)>,
    pub text: String,
    pub slot: bool,
    // TODO do the rest!    
}

impl Action {

    // Static methods to draw the ascii content of ONE box (either with text or not)
    fn add_ascii_box(lines : &mut Vec<String>, content : Option<(u32, u32)>) {

        // Sanity Check:
        if lines.len() != 3 {
            panic!("Ascii box requires five lines as input")
        }

        // Creating the inside text
        // This could be done with a pattern match but it's tricky for me.
        let text_content: String;
        if let Some(range) = content {
            if range.0 == 0 {
                text_content = "FREE ".to_string();
            }
            else if range.0 == range.1 {
                text_content = format!("  {}  ", range.0);
            }
            else {
                text_content = format!(" {}-{} ", range.0, range.1);
            }
        }
        else {
            text_content = "     ".to_string();
        }


        lines[0] += &*"------- ".purple().bold().to_string();
        lines[1] += &*"|".purple().bold().to_string();
        lines[1] += &text_content.bold().to_string();
        lines[1] += &*"| ".purple().bold().to_string();
        lines[2] += &*"------- ".purple().bold().to_string();
    }


    // Displays the full ascii area for ONE action
    pub fn display_ascii(&self) -> String {
        let mut lines = vec![String::new(); 3];

        for elem in &self.cost {
            Action::add_ascii_box(&mut lines, Some(*elem));
        }

        lines[0] += &*format!(" {:<32}", self.name).bold().to_string();
        lines[1] += &*format!(" {:<32}", self.text);
        lines[2] += &*format!(" {:<32}", " ");

        if self.slot {
            Action::add_ascii_box(&mut lines, None);
        }

        lines.join("\n")
    }
}
