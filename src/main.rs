use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;
use std::io::{self, stdin, BufRead, BufReader, Result};
use std::path::Path;

#[derive(Clone)]
struct Task {
    id: u32,
    name: String,
    done: bool,
}

struct List {
    todos: Vec<Task>,
    path: String
}

impl List {
    fn new(pth: String) -> Result<List> {
	let path = Path::new(".todos/todos.txt");
	let file = File::open(&path)?;
	let buf_reader = BufReader::new(file);
	let mut tasks = Vec::new();
	for line in buf_reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(",").collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            let done = parts[2]
		.parse::<bool>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            tasks.push(Task {
		id: id,
		name: parts[1].to_string(),
		done: done,
            });
	}	
	Ok(List {
	    todos: tasks,
	    path: pth
	})
    }
    fn save(&self) -> Result<()> {
	let path = Path::new(&self.path);
	println!("{}", path.display());
	let mut file = File::create(&path)?;
	let todos_str = self.todos.iter()
            .map(|task| format!("{},{},{}", task.id, task.name, task.done))
            .collect::<Vec<String>>()
            .join("\n");
	file.write_all(todos_str.as_bytes())
    }
    fn del(&mut self) -> Result<()> {
	println!("---------------------------------------");
	println!("enter id:");
	println!("---------------------------------------");
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            let id = buffer
		.trim()
		.parse::<u32>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            self.todos.retain(|v| v.id != id);
            let _ = self.save()?;
            println!("{} deleted", id);
	}
	Ok(())
    }
    fn show(&self) {
	println!("---------------------------------------");
	println!("{}", to_str(&self.todos));
    }
    fn add(&mut self) -> Result<()> {
	println!("---------------------------------------");
	println!("enter task ({}):", self.todos.len());
	println!("---------------------------------------");
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	let id = self.todos.last().map_or(1, |task| task.id + 1);
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            self.todos.push(Task {
		id: id,
		name: buffer.trim().to_string(),
		done: false,
            });
            let _ = self.save()?;
            println!("added {}", self.todos[self.todos.len() - 1].name);
	}
	Ok(())
    }
    fn complete(&mut self) -> Result<()> {
	println!("---------------------------------------");
	println!("enter id:");
	println!("---------------------------------------");
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            let id = buffer
		.trim()
		.parse::<u32>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            for task in self.todos.iter_mut() {
		if task.id == id {
                    task.done = !task.done;
                    break;
		}
            }
            let _ = self.save()?;
            println!("{} completed", id);
	}
	Ok(())
    }
    fn reorder(&mut self) -> Result<()> {
	println!("---------------------------------------");
	println!("enter id  index:");
	println!("---------------------------------------");
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
	    let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
	    let mut index: usize = 0;
	    for (ind, task) in self.todos.iter().enumerate() {
		if task.id == id {
		    index = ind;
                    break;
		}
            }
	    let mut dest = parts[1]
		.parse::<usize>()
		.map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
	    if index > dest {
		dest += 1;
	    }
	    
            let task = self.todos.remove(index);
	    self.todos.insert(dest, task);
            let _ = self.save()?;
            println!("{} -> {} reorderd ", id, dest);
	}
	Ok(())
    }
    fn trash(&mut self) -> Result<()> {
	self.todos.retain(|v| v.done == false);
	let _ = self.save()?;
	println!("archive cleared");
	Ok(())
    }

    
}

fn create_dir() -> Result<()> {
    let path = Path::new(".todos");
    fs::create_dir_all(path)?;
    Ok(())
}

fn to_str(todos: &Vec<Task>) -> String {
    let (dones, undones): (Vec<Task>, Vec<Task>) = todos.clone().into_iter().partition(|v| v.done);
    let mut str: String = undones
        .iter()
        .map(|task| {
            format!(
                "[{}] {} {}",
                task.id,
                task.name,
                if task.done == false { "" } else { "(o)" }
            )
        })
        .collect::<Vec<String>>()
        .join("\n");
    if dones.len() > 0 {
        str += "\n==================================[done]\n";
        str += &(dones
		 .iter()
		 .map(|task| {
                     format!(
			 "[{}] {} {}",
			 task.id,
			 task.name,
			 if task.done == false { "" } else { "(o)" }
                     )
		 })
		 .collect::<Vec<String>>()
		 .join("\n"));
    }
    str
}

fn exec(list: &mut List, cmd: &str, prev: &mut String) -> Result<()> {
    match cmd {
        "show" | "s" => {
            list.show();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "add" | "a" => {
            let _ = list.add();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "complete" | "c" => {
            let _ = list.complete();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "reorder" | "r" => {
            let _ = list.reorder();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "trash" | "t" => {
            let _ = list.trash();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "del" | "d" => {
            let _ = list.del();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "exit" | "e" => {
            println!("bye!");
        }
        "" => {
            return exec(list, &prev.clone(), prev);
        }
        _ => {
            println!("command not found...{}, {}", cmd, prev);
            return command(list, prev);
        }
    }
    Ok(())
}

fn command(list: &mut List, prev: &mut String) -> Result<()> {
    println!("---------------------------------------");
    println!("enter command: show, add, complete, trash, reorder, del, exit");
    println!("---------------------------------------");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let cmd = buffer.as_str().trim();
    exec(list, cmd, prev)
}

fn main() -> Result<()> {
    create_dir()?;
    let mut list = List::new(".list/todos.txt".to_string())?;
    let mut prev = "1".to_string();
    let _ = exec(&mut list, "s", &mut prev);
    command(&mut list, &mut prev)
}


