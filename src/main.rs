use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::Write;

fn main () {
	// println!("Please enter a verse:");
	// let mut verse = String::new()
	// std::io::stdin().read_line(&mut verse).unwrap();
	// println!("You entered {}", verse);

    let mut book_abbreviations: HashMap<String, String> = HashMap::new();

    let bible_abbreviations_filename: &str = "Bible_Abbreviations.csv";
    let file = File::open(bible_abbreviations_filename).expect("Failed to open file");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut values = line.split(",");
        book_abbreviations.insert(values.next().unwrap().to_string().to_uppercase(),
                                values.next().unwrap().to_string().to_uppercase());
    }

    loop {

        // TODO: Continue Bible Lookup algorithm (see Python BL project for outline of what steps to do)
        println!("Please enter the reference of the verse you would like to retrieve");
        
        let mut book = String::new();
        let mut chapter = String::new();
        let mut verse = String::new();
        
        println!("Book:");
        io::stdin().read_line(&mut book).expect("Could not read book");
        println!("Chapter:");
        io::stdin().read_line(&mut chapter).expect("Could not read chapter");
        println!("Verse:");
        io::stdin().read_line(&mut verse).expect("Could not read verse");

        book = book.to_uppercase();
        book = "THE BOOK OF ".to_owned() + &book;
        book = book.trim().to_owned();
        
        
        chapter = chapter.trim().to_owned();
        let chap_len = chapter.len();
        chapter = "CHAPTER ".to_owned() + &chapter;
        let mut psalm = "PSALM ".to_owned() + &chapter;
        psalm = psalm.trim().to_owned();

        let bible = File::open("Bible.txt").expect("Failed to open Bible");
        let bible_reader = BufReader::new(bible);

        let mut book_found = false;
        let mut chap_found = false;
        let mut verse_found = false;

        let mut unwrapped;

        for line in bible_reader.lines() {
            unwrapped = line.unwrap_or("Nothing in this line".to_string());
            // book is found?
            if unwrapped == book {
                book_found = true;
            }
            // chapter is found?
            else if book_found && &unwrapped[..chap_len-1] == chapter{
                chap_found = true;
            }
            // psalm is found?
            else if book_found && &unwrapped[..chap_len-1] == chapter {
                chap_found = true;
            }

            // Error cases
            else if book_found && !chap_found && &unwrapped[..10] == "THE BOOK OF" {
                println!("ERROR: This chapter does not exist!");
            }
            else if chap_found && !verse_found && &unwrapped[..4] == "PSALM" {
                println!("ERROR: This verse does not exist!");
                break;
            }
            else if chap_found && !verse_found && &unwrapped[..6] == "CHAPTER" {
                println!("ERROR: This verse does not exist!");
                break;
            }

            else if chap_found && &unwrapped[..verse.len()] == verse {
                verse_found = true;
                println!("{}", unwrapped);
            }
        }
        // Book error case
        if !book_found {
            println!("ERROR: This book does not exist!");
        }

        
        // checking if do again
        println!("Would you like to continue? Y/N ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could not read line");
        
        if input.to_uppercase().starts_with("N") { // neat, much shorter way to check starting character
            return;
        }
        
    }
}
