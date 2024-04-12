

use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use dialoguer::{theme::ColorfulTheme, Select};

struct Todo {
    tasks: Vec<String>,
}

impl Todo {
    fn new() -> Todo {
        Todo { tasks: Vec::new() }
    }
    

    fn load_tasks(&mut self) -> io::Result<()> {
        if let Ok(file) = fs::File::open("tasks.txt") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                self.tasks.push(line?);
            }
        }
        Ok(())
    }

    fn save_tasks(&self) -> io::Result<()> {
        let mut file = fs::File::create("tasks.txt")?;
        for task in &self.tasks {
            writeln!(file, "{}", task)?;
        }
        Ok(())
    }

    fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    fn mark_task_as_finished(&mut self, index: usize) {
        if let Some(task) = self.tasks.get_mut(index) {
            if !task.starts_with("✔️") {
                *task = format!("✔️ {}", task);
            }
        }
    }

    fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    fn list_tasks(&self) {
        println!("Tasks:");
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}. {}", index + 1, task);
        }
    }
}

fn main() -> io::Result<()> {
    let mut todo = Todo::new();
    todo.load_tasks()?;

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose an option")
            .item("Add Task")
            .item("Mark Task as Finished")
            .item("Remove Task")
            .item("List Tasks")
            .item("Exit")
            .interact_opt()
            .unwrap();

        match selection {
            Some(0) => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                let mut task = String::new();
                io::stdin().read_line(&mut task)?;
                todo.add_task(task.trim().to_string());
            }
            Some(1) => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select task to mark as finished")
                    .items(&todo.tasks)
                    .interact_opt()
                    .unwrap();

                match selection {
                    Some(index) => {
                        todo.mark_task_as_finished(index);
                        println!("Task marked as finished: {}", todo.tasks[index]);
                    }
                    None => {
                        println!("No task selected.");
                    }
                }
            }
            Some(2) => {
                print!("Enter task index: ");
                io::stdout().flush().unwrap();
                let mut index_str = String::new();
                io::stdin().read_line(&mut index_str)?;
                if let Ok(index) = index_str.trim().parse::<usize>() {
                    if index > 0 && index <= todo.tasks.len() {
                        todo.remove_task(index - 1);
                    } else {
                        println!("Invalid index");
                    }
                } else {
                    println!("Invalid index");
                }
            }
            Some(3) => {
                todo.list_tasks();
                println!("Press 'b' to go back to the menu.");
                let mut back_input = String::new();
                io::stdin().read_line(&mut back_input)?;
                if back_input.trim() != "b" {
                    println!("Invalid input");
                }
            }
            Some(4) => {
                todo.save_tasks()?;
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }

    Ok(())
}
