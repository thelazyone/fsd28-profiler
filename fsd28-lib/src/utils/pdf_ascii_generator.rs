use regex::Regex;
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
use lopdf::{Document, Object, Dictionary, Stream, content::{Content, Operation}, StringFormat};
fn create_pdf_file(ascii_content: &str, file_name: &str) {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();
    let mut content = Content { operations: vec![] };

    let mut y_position = 280.0; // Starting Y position from top of the page
    let line_height = 3.0; // Adjust line height as needed

    for line in ascii_content.lines() {
        content.operations.push(Operation::new("BT", vec![])); // Begin text object
        content.operations.push(Operation::new("Tf", vec![Object::Name(b"Helvetica".to_vec()), Object::Real(7.0)])); // Set font and size
        content.operations.push(Operation::new("Td", vec![Object::Real(10.0), Object::Real(y_position)])); // Position text
        content.operations.push(Operation::new("Tj", vec![Object::String(line.as_bytes().to_vec(), StringFormat::Literal)])); // Show text
        content.operations.push(Operation::new("ET", vec![])); // End text object
        y_position -= line_height;
    }

    let resources = Dictionary::from_iter(vec![
        ("Font", Object::Dictionary(Dictionary::from_iter(vec![
            ("Helvetica", Object::Name(b"Helvetica".to_vec()))
        ])))
    ]);

    let page = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Page".to_vec())),
        ("Parent", Object::Reference(pages_id)),
        ("Resources", Object::Dictionary(resources)),
        ("MediaBox", Object::Array(vec![0.into(), 0.into(), (210.0).into(), (297.0).into()])),
        ("Contents", Object::Stream(Stream::new(Dictionary::new(), content.encode().unwrap())))
    ]);

    let temp_doc = doc.add_object(page);
    doc.objects.insert(pages_id, Object::Array(vec![Object::Reference(temp_doc)]));
    let mut buffer = BufWriter::new(File::create(file_name).unwrap());
    doc.save_to(&mut buffer).unwrap();
}