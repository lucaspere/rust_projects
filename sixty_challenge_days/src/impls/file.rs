use std::{ffi::OsString, fs, io::Read, path::Path};

pub struct File {
    pub name: OsString,
    pub contents: String,
}

impl File {
    pub fn open_file(filename: &Path) -> Self {
        let mut new_file = File {
            contents: String::new(),
            name: OsString::new(),
        };

        if let Ok(mut file) = fs::File::open(filename) {
            // Safely: unwrapped after open the file
            new_file.name = filename.file_name().unwrap().to_os_string();
            if let Ok(_) = file.read_to_string(&mut new_file.contents) {}
        }

        new_file
    }

    pub fn print_file(self) {
        println!("reading the contents of {:?}", self.name);
        for content in self.contents.lines() {
            println!("{content}");
        }
    }
}
