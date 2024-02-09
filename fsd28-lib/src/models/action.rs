use serde::{Serialize, Deserialize};
use colored::Colorize;

use super::class::Tier;

#[derive(Clone, Serialize, Deserialize)]
pub struct ActionsConfig {
    pub actions: Vec<Action>,
}

// TODO make Vec<(u32, u32)> into a type

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionCost {
    pub goon: Vec<(u32, u32)>,
    pub char: Vec<(u32, u32)>,
    pub hero: Vec<(u32, u32)>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub cost: ActionCost, // TODO this is misleading - cost should be Points, not Activation Dice!
    pub text: String,
    pub slot: bool,
    pub points: u32,
}

impl Action {

    pub fn get_action_cost(&self, tier : &Tier) -> Vec<(u32, u32)> {
        match tier {
            Tier::Goon => self.cost.goon.clone(),
            Tier::Char => self.cost.char.clone(),
            Tier::Hero => self.cost.hero.clone(),
        }
    }

    pub fn get_action_cost_str(&self, tier : &Tier) -> Vec<String> {
        self.get_action_cost(tier).iter().map(|range| {self.range_to_str(range)}).collect()
    }

    fn range_to_str(&self, range: &(u32, u32)) -> String {
        if range.0 == 0 {
            "FREE".to_string()
        }
        else if range.0 == range.1 {
            format!("{}", range.0)
        }
        else {
            format!("{}-{}", range.0, range.1)
        }
    }

    // ASCII ART

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
    pub fn display_ascii(&self, profile_tier: &Tier) -> String {
        let mut lines = vec![String::new(); 3];

        for elem in &self.get_action_cost(profile_tier) {
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
