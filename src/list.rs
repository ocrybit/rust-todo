use std::fs::{File};
use std::io::prelude::*;
use std::io::{self, stdout, stdin, BufRead, BufReader, Result, ErrorKind, Error};
use std::path::Path;
use chrono::prelude::*;

#[derive(Clone, Debug)]
struct Task {
    id: u32,
    name: String,
    done: bool,
    done_at: i64
}

pub struct List {
    todos: Vec<Task>,
    path: String,
    next_id: u32
}

impl List {
    pub fn new(pth: String) -> Result<List> {
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
            tasks.push(Task {
		id: id,
		name: parts[1].to_string(),
		done: done,
		done_at: done_at
            });
	}	
	Ok(List {
	    todos: tasks,
	    path: pth,
	    next_id: next_id + 1
	})
    }
    
    pub fn save(&self) -> Result<()> {
	let path = Path::new(self.path.as_str());
	let mut file = File::create(&path)?;
	let todos_str = self.todos.iter()
            .map(|task| format!("{},{},{},{}", task.id, task.name, task.done, task.done_at))
            .collect::<Vec<String>>()
            .join("\n");
	file.write_all(todos_str.as_bytes())
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
	    self.show();
	}
	Ok(())
    }
    fn to_str(&self) -> String {
	let (mut dones, undones): (Vec<Task>, Vec<Task>) = self.todos.clone().into_iter().partition(|v| v.done);
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

    pub fn show(&self) {
	println!("---------------------------------------");
	println!("{}", self.to_str());
    }
    
    pub fn help(&self) {
	println!("---------------------------------------");
	println!("commands: add | del | complete | reorder | trash | help | exit");
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
		done_at: 0
            });
            let _ = self.save()?;
            println!("added {}", self.todos[self.todos.len() - 1].name);
	    self.show();
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
	    self.show();
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
	    self.show();
	}
	Ok(())
    }
    
    pub fn trash(&mut self) -> Result<()> {
	self.todos.retain(|v| v.done == false);
	let _ = self.save()?;
	println!("archive cleared");
	self.show();
	Ok(())
    }
    
}
