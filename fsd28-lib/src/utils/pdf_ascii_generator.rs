use regex::Regex;
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use crate::Profile;

// Removes the cool looking colour and font options that are used for the console display.
fn strip_ansi_codes(s: &str) -> String {
    let re = Regex::new("\x1B\\[[^m]*m").unwrap();
    re.replace_all(s, "").to_string()
}


// Outputs the PDF file.
pub fn create_pdf_ascii(profiles: Vec<Profile>, file_name: &str){
    let mut combined_profiles = String::new();

    println!("called create_pdf_ascii");

    for profile_pair in profiles.chunks(2) {
        let profile1_ascii = profile_pair[0].display_ascii();
        let profile2_ascii = if profile_pair.len() > 1 {
            profile_pair[1].display_ascii()
        } else {
            "".to_string() // Handle case where there's an odd number of profiles
        };

        println!("two profiles combined");

        combined_profiles += &combine_two_profiles(&strip_ansi_codes(&profile1_ascii), &strip_ansi_codes(&profile2_ascii), 64);
        combined_profiles += "\n\n"; // Add space between rows
    }

    create_pdf_file(&combined_profiles, file_name);
}


// PRIVATE METHODS

// Two profiles are to be displayed per each row. This requires a bit of 
// strings manipulation and padding (both horizontally and vertically).
fn combine_two_profiles(profile1: &str, profile2: &str, pad_length: usize) -> String {
    let lines1 = profile1.lines().map(|line| format!("{:<pad_length$}", line)).collect::<Vec<String>>();
    let lines2 = profile2.lines().map(|line| line.to_string()).collect::<Vec<String>>();

    let max_length = std::cmp::max(lines1.len(), lines2.len());

    (0..max_length)
        .map(|i| {
            let pad1 = " ".repeat(pad_length);
            let line1 = lines1.get(i).unwrap_or(&pad1);
            let pad2 = "".to_string(); 
            let line2 = lines2.get(i).unwrap_or(&pad2);
            format!("{}{}", line1, line2)
        })
        .collect::<Vec<String>>()
        .join("\n")
}

// Generates the actual file, provided a well-formatted string.
fn create_pdf_file(ascii_content: &str, file_name: &str) {
    let ascii_content = strip_ansi_codes(ascii_content);
    let (doc, page1, layer1) = PdfDocument::new("Profile Card", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    let font = doc.add_external_font(File::open("fsd28-lib/data/font/COUR.TTF").unwrap()).unwrap();

    let font_size = 7.0; // Adjust font size as needed
    let line_height = 2.0; // Adjust line height as needed
    let mut y_position = 280.0; // Starting Y position from top of the page

    for line in ascii_content.lines() {
        current_layer.use_text(line, font_size, Mm(10.0), Mm(y_position), &font);
        y_position -= line_height; // Move to the next line
    }

    doc.save(&mut BufWriter::new(File::create(file_name).unwrap())).unwrap();
}