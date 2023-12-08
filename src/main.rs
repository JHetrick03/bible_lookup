/**
 * Description: Looks up a given Bible reference from the user in Bible.txt.
 *              If the book, chapter, or verse does not exist, such is
 *              communicated to the user.
 *              Also prints verses successfully found to verses.txt.
 *              Finally, asks the user if they want to continue searching
 *              for more references.
 * Authors: David Moore & Jeremiah Hetrick
 * Date: 12/07/2023
 * Limitations: None
 */
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::io::Write;
use std::fs::OpenOptions;

fn main () {
    let mut book_abbreviations: HashMap<String, String> = HashMap::new();

    let bible_abbreviations_filename: &str = "Bible_Abbreviations.csv";
    let file = File::open(bible_abbreviations_filename)
    .expect("Failed to open file");

    let mut verse_file = OpenOptions::new().truncate(true).write(true)
    .open("verses.txt").expect("Failed to open verses.txt");

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut values = line.split(",");
        book_abbreviations.insert(values.next().unwrap().to_string().to_uppercase(),
                                values.next().unwrap().to_string().to_uppercase());
    }

    loop {
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

        book = book.trim().to_string().to_uppercase();

        //Checks dictionary. If "book" is an abbreviation, this statement replaces
        //it with the full book name.
        if book_abbreviations.contains_key(&book) {
            let interim_variable = book_abbreviations.get(&book);
            book = interim_variable.unwrap().to_string();
        }

        let just_book = book.trim().to_owned();
        let just_chapter = chapter.trim().to_owned();

        book = "THE BOOK OF ".to_owned() + &book;
        
        let mut psalm = "PSALM ".to_owned() + &chapter;
        psalm = psalm.trim().to_owned();

        chapter = "CHAPTER ".to_owned() + &chapter;
        chapter = chapter.trim().to_owned();

        verse = verse.trim().to_owned();
        let line_verse = verse.len();

        let bible = File::open("Bible.txt").expect("Failed to open Bible");
        let bible_reader = BufReader::new(bible);

        let mut book_found = false;
        let mut chap_found = false;
        let mut verse_found = false;

        let mut curr_line;

        for line in bible_reader.lines() {
            curr_line = line.unwrap_or("Nothing in this line".to_string());
            // book is found?
            if curr_line == book {
                book_found = true;
            }
            else if curr_line.is_empty() {
                
            }
            // chapter is found?
            else if book_found && curr_line == chapter{
                chap_found = true;
            }
            // psalm is found?
            else if book_found && curr_line == psalm { 
                chap_found = true;
            }
            
            // checking for verse
            else if chap_found && &curr_line[..line_verse] == verse {
                verse_found = true;
                println!("\nThe verse requested was:");
                // println!("{} {}:{}", just_book, just_chapter, curr_line);
                prettyprint(just_book.to_owned(),just_chapter
                .to_owned(),curr_line.to_owned());
                write!(verse_file, "{} {}:{}", just_book, just_chapter, curr_line)
                .expect("Error writing to verses.txt");
                write!(verse_file,"\n").expect("Error writing to verses.txt");
                //To do:
                //Format the terminal output to "pretty print" it
                break;
            }

            // Check for error cases
            else if book_found && !chap_found && curr_line.len() >= 11 {
                if &curr_line[..11] == "THE BOOK OF" {
                    break;
                }
            }
            else if chap_found && !verse_found && &curr_line[..5] == "PSALM" {
                break;
            }
            else if chap_found && !verse_found && &curr_line[..7] == "CHAPTER" {
                break;
            }
        }

        // Error messages
        if !book_found {
            println!("ERROR: This book does not exist!");
        }
        else if !chap_found {
            println!("ERROR: This chapter does not exist!");
        }
        else if !verse_found {
            println!("ERROR: This verse does not exist!");
        }
        
        // checking if do again
        println!("\nWould you like to continue? Y/N ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could not read line");
        println!();
        
        if input.to_uppercase().starts_with("N") {
            return;
        }
        
    }
}

// Prints to the screen, limiting lines to 80 characters at most.
// Makes sure that it is not splitting lines in the middle of words.
fn prettyprint (book:String, chapter: String, verse: String) {
    let mut counter = book.len() + chapter.len() + 2;
    let split = verse.split_whitespace();
    print!("{} {}:", book, chapter);

    for next_word in split {
        counter += next_word.len();
        if counter > 80 {
            println!();
            counter = next_word.len();
        }
        print!("{} ", next_word);
    }
    println!();
}