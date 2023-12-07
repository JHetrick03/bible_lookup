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

    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut values = line.split(",");
        book_abbreviations.insert(values.next().unwrap().to_string().to_uppercase(),
                                values.next().unwrap().to_string().to_uppercase());
    }

    loop {
        println!("Would you like to continue? Y/N ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could not read line");
        
        if input.to_uppercase().starts_with("N") { // neat, much shorter way to check starting character
            return;
        }

        // TODO: Continue Bible Lookup algorithm (see Python BL project for outline of what steps to do)
        
        
    }
}
