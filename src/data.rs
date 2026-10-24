// Loading and saving game data.
use crate::board::Board;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
pub const MIN_SHIP: usize = 2;
pub const MAX_SHIP: usize = 5;

#[derive(Debug, Clone, PartialEq)]
pub struct ShipSpec {
    pub name: String,
    pub length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreRecord {
    pub name: String,
    pub score: u32,
}

// WORKED EXAMPLE (provided). Study this before writing parse_ship_line.
// One line of scores.txt looks like:  Ana,83
// Returns None for any line that is not "name,number".
pub fn parse_score(line: &str) -> Option<ScoreRecord> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return None;
    }
    match parts[1].trim().parse::<u32>() {
        Ok(score) => Some(ScoreRecord {
            name: name.to_string(),
            score,
        }),
        Err(_) => None,
    }
}

// PROVIDED: sort highest score first (selection sort).
pub fn sort_scores_desc(records: &mut Vec<ScoreRecord>) {
    for i in 0..records.len() {
        let mut best = i;
        for j in (i + 1)..records.len() {
            if records[j].score > records[best].score {
                best = j;
            }
        }
        records.swap(i, best);
    }
}

// ---------------------------------------------------------------------
// TASKS: write the six functions below (README has the task order).
// Each has a stub that compiles. Remove the todo!() as you implement.
// You will need to add `use` lines at the top as you go.
// ---------------------------------------------------------------------

// Task 1.
// One line of fleet.txt looks like:  Carrier,5
// Return None unless the line has exactly one comma, a non-empty trimmed name,
// and a length that parses as a number from MIN_SHIP to MAX_SHIP.
// Model it on parse_score above.
pub fn parse_ship_line(line: &str) -> Option<ShipSpec> {
    let parts: Vec<&str> = line.trim().split(',').collect();
    if parts.len() != 2 {
        return None;
    }
    let name = parts[0].trim();
    if name.is_empty() {
        return None;
    }
    match parts[1].trim().parse::<usize>() {
        Ok(length) => {
          if length < MIN_SHIP || length > MAX_SHIP {
                return None;
            }
            Some(ShipSpec {
                name: name.to_string(),
                length,
            })
    }
Err(_) => None,
    }	
}

// Task 2.
// Read path and return every valid ship. Blank lines and lines starting with
// '#' are ignored silently. Any other line parse_ship_line rejects is counted;
// if the count is above 0, print

//     Skipped N invalid line(s) in <path>
// to stderr (eprintln!). If the file cannot be read, print the error to
// stderr and return an empty Vec.
pub fn load_fleet(path: &str) -> Vec<ShipSpec> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("{}", e);
            return Vec::new();
        }
    };
    let mut fleet = Vec::new();
    let mut skipped = 0;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_ship_line(line) {
            Some(ship) => fleet.push(ship),
            None => skipped += 1,
        }
    }
    if skipped > 0 {
        eprintln!("Skipped {} invalid line(s) in {}", skipped, path);
    }
    fleet
}
// Task 3.
// Turn the text of a board file into a Board (a 2D array). The text has SIZE
// rows of SIZE characters: '.' is water, 'S' is a ship. Blank lines and lines
// starting with '#' are ignored. Return None for too few rows, too many rows,
// a row of the wrong width, or any other character.
// Start from board::new_board() and fill in board[row][col].
pub fn parse_board(_text: &str) -> Option<Board> {
    todo!("Task 3: parse_board")
}

// Task 4.
// Read the file at path and parse it with parse_board. On a read error, or if
// parse_board returns None, print a message to stderr and return None.
pub fn load_board(_path: &str) -> Option<Board> {
    todo!("Task 4: load_board")
}

// Task 6.
// Append "name,score" as one line to path, creating the file if needed.
// Replace commas in the name with spaces so the line stays parseable.
// Return true if the write succeeded; on failure print the error to stderr
// and return false.
pub fn append_score(path: &str, record: &ScoreRecord) -> bool {
    let name = record.name.replace(',', " ");
    let file = OpenOptions::new().create(true).append(true).open(path);
    match file {
        Ok(mut f) => match writeln!(f, "{},{}", name, record.score) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            false
        }
    }
}
// Task 7.
// Read every valid score line from path. A missing file is normal on the
// first run: return an empty Vec and print nothing. Skip lines that
// parse_score rejects.
pub fn load_scores(path: &str) -> Vec<ScoreRecord> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    let mut records = Vec::new();
    for line in text.lines() {
        match parse_score(line) {
            Some(record) => records.push(record),
            None => {}
        }
    }
    records
}
