use std::fs::{File, OpenOptions};
use std::io::{stdin, BufRead, BufReader, Write};
use std::path::Path;
fn main() {
    let file_path = "notes.txt";
    loop {
        println!("Choose an option:\n1. Add note\n2. View notes\n3. Delete note\n4. Search notes\n5. Quit");
        let mut choice = String::new();
        stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_note(file_path),
            "2" => view_notes(file_path),
            "3" => delete_note(file_path),
            "4" => search_notes(file_path),
            "5" => break,
            _ => println!("Invalid Choice, try again"),
        }
    }
}

fn add_note(file_path: &str) {
    let mut note = String::new();
    println!("Enter note:");
    stdin().read_line(&mut note).unwrap();

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    write!(file, "{}", note).unwrap();
}

fn view_notes(file_path: &str) {
    if !Path::new(file_path).exists() {
        println!("No notes found.");
        return;
    }

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    println!("Your notes: ");
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("{}, {}", index + 1, line);
    }
}

fn delete_note(file_path: &str) {
    if !Path::new(file_path).exists() {
        println!("No notes found.");
        return;
    }

    view_notes(file_path);

    let mut index = String::new();
    println!("Enter the note index to delete:");
    std::io::stdin().read_line(&mut index).unwrap();
    let index: usize = match index.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid index.");
            return;
        }
    };

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    if index == 0 || index > lines.len() {
        println!("Invalid index.");
        return;
    }

    lines.remove(index - 1);

    let mut file = File::create(file_path).unwrap();
    for line in lines {
        writeln!(file, "{}", line).unwrap();
    }

    println!("Note deleted.");
}

fn search_notes(file_path: &str) {
    if !Path::new(file_path).exists() {
        println!("No notes found.");
        return;
    }

    println!("Enter a keyword or phrase to search:");
    let mut keyword = String::new();
    stdin().read_line(&mut keyword).unwrap();
    let keyword = keyword.trim().to_string();

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    println!("Notes containing {keyword}");

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.contains(&keyword) {
            println!("{}", line);
            count += 1;
        }
    }

    println!("Found {count} notes")
}
