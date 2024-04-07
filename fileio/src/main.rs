use std::io::{self, Read, Write};
use std::process::Command;
fn clearconsole(){
    let clear = Command::new("cmd").arg("/c").arg("cls").status();
    clear.unwrap();
}
fn writefile() {
    let mut file = std::fs::File::create("file.txt").unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    file.write_all(input.as_bytes()).unwrap();
}

fn readfile() {
    let mut file = std::fs::File::open("file.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}

fn main() {
    loop {
        println!("Select operation:");
        println!("1. Write to file");
        println!("2. Read from file");
        println!("3. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                clearconsole();
                println!("Enter text to write to file:");
                writefile();
            }
            "2" => {
                clearconsole();
                println!("Contents of the file:");
                readfile();
            }   
            "3" => {
                println!("Exiting program...");
                break;
            }
            _ => println!("Invalid choice. Please select a number between 1 and 4."),
        }
    }
}
