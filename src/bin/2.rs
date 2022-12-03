use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    let mut scoremap = HashMap::<String, i64>::new();
    scoremap.insert("A X".to_string(), 3+1);
    scoremap.insert("B X".to_string(), 0+1);
    scoremap.insert("C X".to_string(), 6+1);
    scoremap.insert("A Y".to_string(), 6+2);
    scoremap.insert("B Y".to_string(), 3+2);
    scoremap.insert("C Y".to_string(), 0+2);
    scoremap.insert("A Z".to_string(), 0+3);
    scoremap.insert("B Z".to_string(), 6+3);
    scoremap.insert("C Z".to_string(), 3+3);

    let mut scoremap2 = HashMap::<String, i64>::new();
    scoremap2.insert("A X".to_string(), 0+3);
    scoremap2.insert("B X".to_string(), 0+1);
    scoremap2.insert("C X".to_string(), 0+2);
    scoremap2.insert("A Y".to_string(), 3+1);
    scoremap2.insert("B Y".to_string(), 3+2);
    scoremap2.insert("C Y".to_string(), 3+3);
    scoremap2.insert("A Z".to_string(), 6+2);
    scoremap2.insert("B Z".to_string(), 6+3);
    scoremap2.insert("C Z".to_string(), 6+1);


    let mut score:i64 = 0;
    let mut score2:i64 = 0;

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                score += scoremap[&ip];
                score2 += scoremap2[&ip];
            }
        }
    }
    println!("{}",score);
    println!("{}",score2);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
