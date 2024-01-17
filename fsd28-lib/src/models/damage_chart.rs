
use colored::Colorize;
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
        DamageChart {intervals: vec![(1, Color::Red, "DEAD".to_string()), (2, Color::Yellow, "MOV ".to_string()), (2, Color::Yellow, "ARM ".to_string()), (1, Color::Green, "PIN ".to_string()),]}
    }

    // TODO implement proper new

    // TODO implement display_ascii
    // Displaying as ASCII
    // Sintax is 
    //
    //    1        2        3        4        5        6   
    //| DEAD  || DEAD  || DEAD  || DEAD  || DEAD  || DEAD |
    //
    //
    pub fn display_ascii(&self) -> String {
        let mut out_string = "----------------------------------------------------------\n| ".blue().bold().to_string();
        out_string += "    1        2        3        4        5        6    ";
        out_string += &*" |\n| ".blue().bold().to_string();
        for element in &self.intervals {
            out_string += "|  ";
            match element.1 {
                Color::Red => out_string += &*element.2.red().bold().to_string(),
                Color::Yellow => out_string += &*element.2.yellow().bold().to_string(),
                Color::Green => out_string +=&*element.2.green().bold().to_string(),
            };
            for _ in 1..element.0 {
                out_string += "         ";
            }
            out_string += " |";
        }
        out_string += &*" |\n".blue().bold().to_string();
        out_string += &*"----------------------------------------------------------".blue().bold().to_string();
        out_string
    }

    // TODO implement display_png
}

