pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn checkout_book(&mut self, title: String) {
        for book in self.books {
            if book.title == title {
                todo!("tomorrow")
            }
        }
    }

    pub fn list_available_books(&self) {}
}

pub struct Book {
    title: String,
}
