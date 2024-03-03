use std::{
    ffi::OsString,
    fs,
    io::{Error, Read},
    path::Path,
};

pub struct File {
    pub name: OsString,
    pub contents: String,
}

impl File {
    pub fn open_file(filename: &Path) -> Result<Self, Error> {
        let mut new_file = File {
            contents: String::new(),
            name: OsString::new(),
        };

        let mut file = fs::File::open(filename)?;
        new_file.name = filename.file_name().unwrap_or_default().to_os_string();
        file.read_to_string(&mut new_file.contents)?;

        Ok(new_file)
    }

    pub fn print_file(self) {
        println!("reading the contents of {:?}", self.name);
        for content in self.contents.lines() {
            println!("{content}");
        }
    }
}
