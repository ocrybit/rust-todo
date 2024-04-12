use std::fs::{File};
use std::io::prelude::*;
use std::io::{Result, BufRead, BufReader, ErrorKind, Error};
use std::path::Path;
use chrono::prelude::*;
use crate::libs::storage::{ Task, Todos, Storage };
use crate::libs::utils::{ bar2, bar, to_str, get_value, get_values, get_values3 };
use rustyline::{Result as ResultRL};

impl Todos {
    pub fn new(pth: String) -> Result<Todos> {
	Todos::load(pth)
    }
    
    pub fn del(&mut self, _id: &str) -> ResultRL<()> {
	bar();
	let __id = get_value("enter id: ", "", _id)?;
	if __id == "" {
            println!("cancel");
	} else {
            let id = __id
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
	to_str(self.todos.clone(), tag)
    }

    pub fn show(&self, tag: &str) {
	bar2("tasks");
	println!("{}", self.to_str(tag));
    }
    
    pub fn count(&self, tag: &str) -> String {
	let mut todos = self.todos.clone();
	if tag == "_" || tag == "" {
	    todos.retain(|v| v.lists.len() == 0 || v.lists.contains(&("_".to_string())));
	} else if tag != "" {
	    todos.retain(|v| v.lists.contains(&tag.to_string()));
	}
	let (dones, undones): (Vec<Task>, Vec<Task>) = todos.into_iter().partition(|v| v.done);
	format!("{} : {}", dones.len(), undones.len())
    }
    
    pub fn help(&self) {
	bar();
	println!("[todos-commands] show | add | del | complete | reorder | list | trash");
	println!("[lists-commands] list-show | list-add | list-del");
	println!("[other-commands] help | exit");
    }

    pub fn add(&mut self, _id: &str, _id2: &str) -> ResultRL<()> {
	bar();
	let (__id, __id2) = get_values("enter task, tags: ", "enter tags: ", "", _id, _id2)?;
	if __id == "" {
            println!("cancel");
	} else {
	    let id = self.next_id;
	    self.next_id += 1;
	    let tags: Vec<String> = if __id2 == "" {
		vec![]
	    } else {
		__id2.trim().split("|").map(|s| s.to_string()).collect()
	    };
            self.todos.push(Task {
		id: id,
		name: __id.trim().to_string(),
		done: false,
		done_at: 0,
		lists: tags.clone()
            });
            let _ = self.save()?;
            println!("added {}", self.todos[self.todos.len() - 1].name);
	    self.show(if __id2 != "" { tags[0].as_str() } else { "" });
	}
	Ok(())
    }

    pub fn edit(&mut self, _id: &str, _id2: &str) -> ResultRL<()> {
	bar();
	let (__id, __id2) = get_values("enter id, task: ", "enter task: ", "", _id, _id2)?;
	if __id == "" || __id2 == "" {
            println!("cancel");
	} else {
	    let id = __id
		.trim()
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    let name = __id2.trim().to_string();
	    self.todos.iter().position(|x| x.id == id).map_or_else(
		|| println!("not found"),
		| ind | {
		    println!("edited {}: {}", id, name);		    
		    self.todos[ind].name = name.clone()
		}
	    );
            let _ = self.save()?;
	    self.show("");
	}
	Ok(())
    }
    
    pub fn complete(&mut self, _id: &str) -> ResultRL<()> {
	bar();
	let __id = get_value("enter id: ", "", _id)?;
	if __id == "" {
            println!("cancel");
	} else {
            let id = __id
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

    pub fn list(&mut self, _id: &str, _id2: &str) -> ResultRL<()> {
	bar();
	let (__id, __id2) = get_values("enter id, list: ", "enter list: ", "", _id, _id2)?;
	if __id == "" || __id2 == "" {
            println!("cancel");
	} else {
            let id = __id
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            let tag = __id2.trim();
            for task in self.todos.iter_mut() {
		if task.id == id {
		    task.lists.push(tag.to_string());
                    break;
		}
            }
            let _ = self.save()?;
            println!("{} added to list {}", id, tag);
	    self.show(tag);
	}
	Ok(())
    }

    pub fn unlist(&mut self, _id: &str, _id2: &str) -> ResultRL<()> {
	bar();
	let (__id, __id2) = get_values("enter id, list: ", "enter list: ", "", _id, _id2)?;
	if __id == "" || __id2 == ""{
            println!("cancel");
	} else {
            let id = __id
		.parse::<u32>()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
            let tag = __id2.trim();
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
	    self.show(tag);
	}
	Ok(())
    }
    
    pub fn reorder(&mut self, _id: &str, _id2: &str, _id3: &str) -> ResultRL<()> {
	bar();
	let (__id, __id2, __id3) = get_values3("enter id, index, tag", "enter index, tag: ", "enter tag: ", "", _id, _id2, _id3)?;
	if __id == "" {	
            println!("cancel");
	} else {
	    let tag = __id3.trim().to_string();
	    let mut _todos : Vec<Task> = self.todos.clone();
	    if __id3 != "" {
		_todos.retain(|v| v.lists.contains(&tag.to_string()));
	    }
	    
            let id = __id
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
	    if __id2 != "" {
		dest = _id2
		    .parse::<usize>()
		    .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
	    }
	    println!("{}", dest);
	    let mut done_count = 0usize;
	    let mut undone_count = 0usize;
	    let mut dest2 = if let Some(last) = _todos.last() { last.id } else { 0 } ;
	    for (_, task) in _todos.iter().enumerate() {
		if is_done == true {
		    if task.done == true && done_count == dest {
			dest2 = task.id;
			break;
		    }
		}else{
		    
		    if task.done == false && undone_count == dest {
			dest2 = task.id;
			break;
		    }
		}
		if task.done == true {
		    done_count += 1;
		} else {
		    undone_count += 1;
		}
            }
	    let mut dest3 = if let Some(d) = self.todos.iter().position(|x| x.id == dest2) { d } else { 0 };
	    done_count = 0;
	    undone_count = 0;
	    for (ind, task) in self.todos.iter().enumerate() {
		if is_done == true {
		    if task.done == true && done_count == dest3 {
			dest3 = ind;
			break;
		    }
		}else{
		    if task.done == false && undone_count == dest3 {
			dest3 = ind;
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
	    self.todos.insert(dest3, task);
            let _ = self.save()?;
            println!("{} -> {} : {} reorderd ", id, dest, dest3);
	    self.show(_id3);
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
		let mut _lists: Vec<&str> = parts[4].split("|").collect();
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
