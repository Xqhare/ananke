use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use crate::task::Task;

#[derive(Debug, Clone)]
pub struct List {
    tasks: Vec<Task>,
}

impl List {
    pub fn open(filename: &str) -> Self {
        let file_lines = Self::read_lines(filename);
        let mut output: Vec<Task> = Vec::new();
        if let Ok(lines) = file_lines {
            
            for line in lines {
                if let Ok(task) = line {
                    let made_task: Task = Task::new(task);
                    output.push(made_task);
                }
            }
        
        }
        return List{tasks: output};
    }
    pub fn return_all_tasks(&self) -> Vec<Task> {
        let output = self.tasks.clone();
        return output;
    }
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn print_all(list: List) {
        let mut stupid_tracker = 0;
        for task in list.tasks {
            println!("Task: -- {stupid_tracker}");
            task.debug_print();
            stupid_tracker += 1;
        }
    }
}
