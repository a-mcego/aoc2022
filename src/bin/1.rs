use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use priority_queue::PriorityQueue;

fn main() {

    let mut pq = PriorityQueue::new();

    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("1.txt") {
        let mut current_sum = 0;
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.len() == 0 {
                    pq.push(current_sum,current_sum);
                    current_sum = 0;
                }
                else {
                    current_sum += ip.parse::<i32>().unwrap();
                }
            }
        }
        pq.push(current_sum,current_sum);
    }
    println!("{}",pq.peek().unwrap().0);

    let mut three_best = 0;

    three_best += pq.pop().unwrap().0;
    three_best += pq.pop().unwrap().0;
    three_best += pq.pop().unwrap().0;

    println!("{}",three_best);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
