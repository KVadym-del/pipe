use std::io::{ Read, Write, stdout };
use std::fs::{ read_to_string };
use termion::screen::IntoAlternateScreen;

#[allow(dead_code)]
pub fn editor(file_path: &str) {
    let mut file_buffer = String::new();

    match read_to_string(file_path) {
        Ok(content) => {
            file_buffer = content;
        }
        Err(e) => {
            eprintln!("ERROR: Failed to read [{file}] file => {}", e, file = file_path);
        }
    }

    {
        let mut screen = stdout().into_alternate_screen().unwrap();
        write!(screen, "Writing to alternate screen!").unwrap();
        screen.flush().unwrap();
    }
    println!("Writing to main screen.");


}
