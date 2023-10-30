use std::env;

mod gui;
mod list;
mod task;

const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    println!("Welcome to {NAME} by {AUTHOR}, v. {VERSION}");
    // read_file();
    gui::main();
}

fn read_file() {
    let path: &str = "./todo-test.txt";
    let todo_list: list::List = list::List::open(path);
    list::List::print_all(todo_list)
    /* if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(task) = line {
                println!("l: {task}");
            }
        }
    } */
}
