pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn checkout_book(&mut self, title: String) {
        for book in &mut self.books {
            if book.title == title {
                book.checkout_out = true;
            }
        }
    }

    pub fn list_available_books(&self) {
        for book in &self.books {
            if book.checkout_out {
                println!("Title: {}", book.title);
            }
        }
    }
}

pub struct Book {
    pub title: String,
    pub checkout_out: bool,
}
