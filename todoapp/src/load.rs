use std::{fs, io};
use std::io::BufReader;

pub fn load_tasks(&mut self) -> io::Result<()> {
    if let Ok(file) = fs::File::open("tasks.txt") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            self.tasks.push(line?);
        }
    }
    Ok(())
}