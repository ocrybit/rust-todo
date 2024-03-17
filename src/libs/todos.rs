use std::fs::{File};
use std::io::prelude::*;
use std::io::{self, stdout, stdin, BufRead, BufReader, Result, ErrorKind, Error};
use std::path::Path;
use chrono::prelude::*;
use crate::libs::storage::Storage;

#[derive(Clone, Debug)]
struct Task {
    id: u32,
    name: String,
    done: bool,
    done_at: i64,
    lists: Vec<String>
}

pub struct Todos {
    todos: Vec<Task>,
    path: String,
    next_id: u32
}

impl Todos {
    pub fn new(pth: String) -> Result<Todos> {
	Todos::load(pth)
    }
    pub fn del(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter id: ");
	io::stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            let id = buffer
		.trim()
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            self.todos.retain(|v| v.id != id);
            let _ = self.save()?;
            println!("{} deleted", id);
	    self.show("");
	}
	Ok(())
    }
    fn to_str(&self, tag: &str) -> String {
	let mut todos2 = self.todos.clone();
	if tag != "" {
	    todos2.retain(|v| v.lists.contains(&tag.to_string()));
	}
	let (mut dones, undones): (Vec<Task>, Vec<Task>) = todos2.into_iter().partition(|v| v.done);
	let mut str: String = undones
            .iter()
            .map(|task| {
		let mut lists = task.lists.iter().map(|l| format!("{}", l)).collect::<Vec<String>>().join("|");
		lists = format!("<{}>",lists);
		format!(
                    "[{}] {} {}",
                    task.id,
                    task.name,
		    if task.lists.len() == 0 || (task.lists.len() == 1 && task.lists[0] == "") { "".to_string() } else { lists }
		)
            })
            .collect::<Vec<String>>()
            .join("\n");
	if dones.len() > 0 {
            str += "\n==================================[done]\n";
	    dones.sort_by_key(|v| v.done_at);
            str += &(dones
		     .iter()
		     .map(|task| {
			 let ts = task.done_at / 1000;
			 let datetime: DateTime<Utc> = Utc.timestamp_opt(ts, 0).unwrap();
			 let ts2 = datetime.format("%m/%d %H:%M").to_string();
			 format!(
			     "[{}] {} ({})",
			     task.id,
			     task.name,
			     ts2
			 )
		     })
		     .collect::<Vec<String>>()
		     .join("\n"));
	}
	str
    }

    pub fn show(&self, tag: &str) {
	println!("[tasks]---------------------------------------");
	println!("{}", self.to_str(tag));
    }
    
    pub fn help(&self) {
	println!("---------------------------------------");
	println!("[todos-commands] show | add | del | complete | reorder | trash");
	println!("[lists-commands] list-show | list-add | list-del");
	println!("[other-commands] help | exit");
    }

    pub fn add(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter task: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	let id = self.next_id;
	self.next_id += 1;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            self.todos.push(Task {
		id: id,
		name: buffer.trim().to_string(),
		done: false,
		done_at: 0,
		lists: vec![]
            });
            let _ = self.save()?;
            println!("added {}", self.todos[self.todos.len() - 1].name);
	    self.show("");
	}
	Ok(())
    }

    
    pub fn complete(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter id: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            let id = buffer
		.trim()
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            for task in self.todos.iter_mut() {
		if task.id == id {
                    task.done = !task.done;
		    if task.done == false {
			task.done_at = 0
		    }else{
			task.done_at = Utc::now().timestamp_millis();
		    }
                    break;
		}
            }
            let _ = self.save()?;
            println!("{} completed", id);
	    self.show("");
	}
	Ok(())
    }

    pub fn list(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter id list: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
	    let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            let tag = parts[1].trim();
            for task in self.todos.iter_mut() {
		if task.id == id {
		    task.lists.push(tag.to_string());
                    break;
		}
            }
            let _ = self.save()?;
            println!("{} added to list {}", id, tag);
	    self.show("");
	}
	Ok(())
    }

    pub fn unlist(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter id list: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
	    let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            let tag = parts[1].trim();
            for task in self.todos.iter_mut() {
		if task.id == id {
		    let mut exists = false;
		    let mut index  = 0;
		    for (ind, l) in task.lists.iter().enumerate() {
			if l == tag {
			    index = ind;
			    exists = true;
			    break;
			}
		    }
		    if exists == true { task.lists.remove(index); }
                    break;
		}
            }
            let _ = self.save()?;
            println!("{} removed from list {}", id, tag);
	    self.show("");
	}
	Ok(())
    }
    
    pub fn reorder(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter id index: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
	    let parts: Vec<&str> = buffer.trim().split_whitespace().collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    let mut index: usize = 0;
	    let mut is_done = false;
	    for (ind, task) in self.todos.iter().enumerate() {
		if task.id == id {
		    index = ind;
		    is_done = task.done;
		}
            }
	    let mut dest = 0;
	    if parts.len() > 1 {
		dest = parts[1]
		    .parse::<usize>()
		    .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    }
	    let mut done_count = 0usize;
	    let mut undone_count = 0usize;
	    for (ind, task) in self.todos.iter().enumerate() {
		if is_done == true {
		    if task.done == true && done_count == dest {
			dest = ind;
			break;
		    }
		}else{
		    if task.done == false && undone_count == dest {
			dest = ind;
			break;
		    }
		}
		if task.done == true {
		    done_count += 1;
		} else {
		    undone_count += 1;
		}
            }
            let task = self.todos.remove(index);
	    self.todos.insert(dest, task);
            let _ = self.save()?;
            println!("{} -> {} reorderd ", id, dest);
	    self.show("");
	}
	Ok(())
    }
    
    pub fn trash(&mut self) -> Result<()> {
	self.todos.retain(|v| v.done == false);
	let _ = self.save()?;
	println!("archive cleared");
	self.show("");
	Ok(())
    }
    
}

impl Storage<Todos> for Todos {
    fn save(&self) -> Result<()> {
	let path = Path::new(self.path.as_str());
	let mut file = File::create(&path)?;
	let todos_str = self.todos.iter()
            .map(|task| {
		let lists = task.lists.iter().map(|l| format!("{}", l)).collect::<Vec<String>>().join("|");
		format!("{},{},{},{},{}", task.id, task.name, task.done, task.done_at, lists)
	    })
            .collect::<Vec<String>>()
            .join("\n");
	file.write_all(todos_str.as_bytes())
    }
    fn load(pth: String) -> Result<Todos> {
	let path = Path::new(pth.as_str());
	let file = File::open(&path)?;
	let buf_reader = BufReader::new(file);
	let mut tasks = Vec::new();
	let mut next_id = 0u32;
	for line in buf_reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(",").collect();
            let id = parts[0]
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    if id > next_id { next_id = id };
            let done = parts[2]
		.parse::<bool>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    let mut done_at = 0i64;
	    if parts.len() > 3 {
		done_at = parts[3]
		    .parse::<i64>()
		    .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;

	    }
	    let mut lists : Vec<String> = vec![];
	    if parts.len() > 4  && parts[4] != "" {
		let mut _lists: Vec<&str> = parts[4].split(",").collect();
		_lists.retain(|v| *v != "");
		for l in _lists.iter(){
		    lists.push(l.to_string());
		}
	    }
            tasks.push(Task {
		id: id,
		name: parts[1].to_string(),
		done: done,
		done_at: done_at,
		lists: lists
            });
	}	
	Ok(Todos {
	    todos: tasks,
	    path: pth,
	    next_id: next_id + 1
	})
    }
}
