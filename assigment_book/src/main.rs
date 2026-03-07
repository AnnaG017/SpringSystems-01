use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let mut file = File::create(filename).unwrap();
    writeln!(file,"book:").unwrap();
   for book in books{
        writeln!(file, "{},{},{}",book.title,book.author,book.year).unwrap();
   }

}

fn load_books(filename: &str) -> Vec<Book> {
    // Open the file for reading
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books = Vec::new();

    // Read each line
    for (i, line) in reader.lines().enumerate() {
        // Skips header line
        if i == 0 {
            continue;
        }

        let line = line.unwrap();

        // Split by commas
        let mut parts = line.split(',');
        // Take each value from the split line: title, author, and year 
        let title = parts.next().unwrap().to_string();
        let author = parts.next().unwrap().to_string();
        let year = parts.next().unwrap().parse::<u16>().unwrap();

        books.push(Book { title, author, year });
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}