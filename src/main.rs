#![allow(dead_code)]
use std::path::Path;
use std::{env, path::PathBuf};
use std::fs::File;
use serde::{Serialize, Deserialize};
use serde_json;

struct TodoList {
    items: Vec<TodoItem>,
    file: Option<File>
}

impl TodoList {
    fn initialize(&mut self) {
        File::create("todo.json").unwrap();
    }

    fn parse_args(&mut self, args: Vec<String>) {
        if args[1] == String::from("add") {
            if args.len() >= 3 {
                self.add_command(args);        
            }
            else {
                panic!("No command after add");
            }
        }

        else {
            println!("Unrecognized command");
        }
        
    }

    //need to add to json file
    fn add_command(&mut self, args: Vec<String>) {
        let item = TodoItem::new(1, args[2].clone(), None, false);
        println!("{:?}", item);
        self.items.push(item);
        
    }

    //deserialize the json file
    fn show(&mut self) {

    }

    fn write_to_json(&mut self, item: TodoItem) {
//        serde_json::to_writer()
    }

}
 
#[derive(Debug, Serialize, Deserialize)]
struct TodoItem {
    id: i32,
    title: String,
    #[serde(rename = "additionalNotes")]
    additional_notes: Option<String>,
    completed: bool,
}

impl TodoItem {
    fn new(id: i32, title: String, additional_notes: Option<String>, completed: bool) -> Self {
        Self {
            id,
            title,
            additional_notes,
            completed
        }
    }
}

#[derive(PartialEq, PartialOrd)]
enum Commands {
    Add,
    Complete,
    Remove,
    Show
}

fn find_todo(path: &Path) -> Option<File> {
    if path.exists() {
        println!("Found path!");
        let file = File::open(path).unwrap();
        return Some(file)
    }
    None
}

fn main() {
    let mut todo = TodoList {items: vec![], file: None};
    let mut dir = env::current_dir().unwrap();
    let mut dir = dir.to_string_lossy().to_string();
    dir.push_str("/todo.json");
    let path = Path::new(&dir);
    println!("Curr directory: {:?}", path);

    if let Some(f) = find_todo(path) {
        todo.file = Some(f);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        panic!("Not enough arguments")
    }

    for x in &args {
        println!("Arg: {}", x);
    }

//instead need to check if todo.json exists
    if args[1] == String::from("init") {
        if todo.file.is_some() {
            println!("Already initialized!");
        }
        else {
            todo.initialize();
            println!("Initializing!");
        }
    }
    else if !todo.file.is_some() {
        panic!("Error: need to initialize todo.  Do todo init");
    }
    else {
        todo.parse_args(args);
        println!("Parsing Arguments");
    }
}
//todo add item optional<description> optional<-c> (complete)
//todo complete id
//todo remove id
//todo show
