# Day 1: Rust Ownership Foundations - 15-02-2024

## Exercises

### Understanding Ownership

Read and Absorb: Read Chapters 1 through 4 of "The Rust Programming Language" book (https://doc.rust-lang.org/book/). Don't just skim. Focus on these points:

- Why ownership matters: What types of bugs does it prevent (memory leaks, double frees, etc.)? Contrast this to garbage collection-based languages you might know.
- The Rules: When is a value dropped? How do references/borrowing interact with this? Can you have multiple mutable references at the same time?
- Mental Models: Think of how these rules would work in different theoretical scenarios, even before writing code.

### Practical Implementation

#### Exercise 1: Transfer of Ownership

- Create a simple struct to represent a `File` with fields like name (`String`) and contents (`String`).
- Write a function `open_file` that takes a filename, reads the content, and returns a `File` struct.
- Write a `print_file` function that takes ownership of a File and prints its contents.
  In your main function, call `open_file`, and then pass the resulting `File` to `print_file`. Experiment with trying to use the `File` again afterward - the compiler will teach you!

#### Exercise 2: References & Borrowing

- Create a struct to represent a ``Library`` that has a books field (a vector of Book structs – you might implement Book tomorrow).
- Write a ``checkout_book`` function on ``Library``. It should take a `&mut` self and the title of a book, find the book in the library and change a flag representing its “checked out” status.
- Write a `list_available_books` function that uses an immutable reference (`&self`) and prints the titles of only the available books.
- Explore: Try making some mistakes - can you have a mutable and immutable borrow of the same library at the same time? Why or why not?

#### Show Me
Your Code: Post your Rust code with attempts at both exercises. It's totally okay if you don't get everything right the first time!
Your Explanation: In your own words, explain these concepts to me:
- Why a value can only have one owner at a time.
- The key difference between a regular variable and a reference.
- Scenarios where you would likely use a mutable vs. an immutable reference

## Answers

### Understanding Ownership

Ownership is an important technique to guarantee safe memory management in the Rust language. It prevents **_double free_** by the rule of dropping a value in final of its scope. Another bug is **_data race_** which occurs when two variables can mutate the same value. Ownership prevents this by not allowing two variables carrying multiple mutable references to the same value. The rule is: that one immutable value can have multiple references, but a mutable value can only have one mutable reference. Ownership also prevents the bug **_Dangling references_** at compile time checking if a variable is trying to get a reference from a value that has already dropped. The GC in JavaScript handles this automatically in the background by checking periodically if a variable is referenced by any others.


### Exercise 1: Transfer of Ownership
```rs
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

```

### Exercise 2: References & Borrowing
```rs
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

```
#### Show Me
A value can only have one owner at a time because the Ownership technique that Rust use to request from the memory allocator and the returning the memory to the allocator.
A regular variable points to the data type that was assigned to it. The reference points to the memory where the data is allocated.
I could use a mutable just for read its contents, like the print_file does. I will use immutable when wants to modify or assign some value.

