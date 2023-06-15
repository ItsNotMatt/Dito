use std::io::{Write, Read};
use std::path::Path;
use std::env;
use std::fs::{File, OpenOptions};
use serde::{Serialize, Deserialize};
use serde_json;

struct TodoList {
    items: Vec<TodoItem>,
    file: Option<File>,
    path: Option<String>
}

impl TodoList {
    fn initialize(&mut self) {
        File::create("todo.json").unwrap();
    }

    fn parse_args(&mut self, args: Vec<String>) {
        let command = args[1].clone();
        match command.as_str() {
            "add" => {
                if args.len() >= 3 {
                    self.add_todo(args);        
                }
                else {
                    panic!("No command after add");
                }
            },
            "show" => {
                if args.len() > 2 {
                    if args[2] == "-c" {
                        self.show_completed_todos();
                    }
                    else if args[2] == "-i" {
                        self.show_incompleted_todos();
                    }
                    return
                }
                self.show_todos();
            },
            "complete" => {
                let id = args[2].clone();
                if let Ok(u) = id.parse::<usize>() {
                    self.complete_todo(u);
                }
                else {
                    println!("Second arg is not a valid usize");
                }
            }
            "remove" => {
                self.get_todos();//have to call it before so we can check size 

                let id = args[2].clone();
                if let Ok(u) = id.parse::<usize>() {
                    if u > self.items.len() || u < 1 {
                        println!("Id is too high or too low");
                    }
                    else {
                        println!("Removing item");
                        self.remove_todo(u);
                    }
                }
                else {
                    println!("Second arg is not a valid usize");
                }
            }           
            "help" => {
                println!("Commands: add, show, complete, remove, help");
                println!(r#"todo add "Name of new todo item in quotes""#);
                println!("todo show  |  Optionally add todo show -c (for complete) or -i (for incomplete");
                println!("todo complete Id");
                println!("todo remove Id");
            }

            _ => {
                println!("Unrecognized command");
            },
        };
                    
    }

    fn add_todo(&mut self, args: Vec<String>) {
        self.get_todos(); //brings all items from json to the struct
                          
        let item = TodoItem::new(self.items.len() + 1, args[2].clone(), None, false);
        self.items.push(item.clone());
        println!("Adding: {} to the list", item.title);
        self.post_todos();
    }

    fn show_todos(&mut self) {
        self.get_todos();
        for todo in &self.items {
            if todo.completed {
                println!("[✅]{}: {}", todo.id, todo.title);
            }
            else {
                println!("[❌]{}: {}", todo.id, todo.title);
            }
        } 
    }
    
    fn show_completed_todos(&mut self) {
        self.get_todos();
        for todo in &self.items {
            if todo.completed {
                println!("[✅]{}: {}", todo.id, todo.title);
            }
        }
    }

    fn show_incompleted_todos(&mut self) {
        self.get_todos();
        for todo in &self.items {
            if !todo.completed {
                println!("[❌]{}: {}", todo.id, todo.title);
            }
        }
    }

    fn complete_todo(&mut self, id: usize) {
        self.get_todos();
        for i in 0..self.items.len() {
            if self.items[i].id == id {
                self.items[i].completed = true;
                println!("Completed todo item: {}", self.items[i].title);
                self.post_todos();
            }
        }
    }

    fn remove_todo(&mut self, id: usize) {
        for i in 0..self.items.len() {
            if self.items[i].id == id {
                println!("Removing: {} from the list", self.items[i].title);
                self.items.remove(i);
                for x in 0..self.items.len() {
                    self.items[x].id = x + 1;
                }
                self.post_todos();
                break
            }
        }
    }

    //deserialize the json file
    fn get_todos(&mut self) {
        let mut content = String::new();
        self.file.as_ref().expect("Cant read").read_to_string(&mut content).unwrap();
        //println!("Content: {}", content);
        if let Ok(todos) = serde_json::from_str::<Vec<TodoItem>>(&content) {
            for i in todos {
                self.items.push(i);
            }
        }
        else if let Ok(todo) = serde_json::from_str::<TodoItem>(&content) {
            self.items.push(todo);
        }
        else if content.is_empty() {
            //println!("No content");
        }
        else {
            println!("Err getting todo item from json");
        }
    }

    fn post_todos(&mut self) {
        if !self.items.is_empty() {
            if let Ok(json) = serde_json::to_string_pretty(&self.items) {
                let s = self.path.clone().unwrap();
                let path = Path::new(&s);
                self.file = Some(OpenOptions::new().write(true).truncate(true).open(&path).expect("Err opening file"));
                self.file.as_ref().expect("Cant write to file").write_all(json.as_bytes()).unwrap();
            }
        }
    }

}
 
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TodoItem {
    id: usize,
    title: String,
    #[serde(rename = "additionalNotes")]
    additional_notes: Option<String>,
    completed: bool,
}

impl TodoItem {
    fn new(id: usize, title: String, additional_notes: Option<String>, completed: bool) -> Self {
        Self {
            id,
            title,
            additional_notes,
            completed
        }
    }
}

fn find_todo(path: &Path) -> Option<File> {
    if path.exists() {
        let file = OpenOptions::new().read(true).write(true).open(path).unwrap();
        return Some(file)
    }
    None
}

fn main() {
    let mut todo = TodoList {items: vec![], file: None, path: None};
    let dir = env::current_dir().unwrap();
    let mut dir = dir.to_string_lossy().to_string();
    dir.push_str("/todo.json");
    let path = Path::new(&dir);

    if let Some(f) = find_todo(path) {
        todo.file = Some(f);
        todo.path = Some(dir);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Dito Version: 1.0");
        std::process::exit(0);
    }

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
    }
}
