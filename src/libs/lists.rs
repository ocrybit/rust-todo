use std::fs::{File};
use std::io::prelude::*;
use std::io::{self, stdout, stdin, Result, ErrorKind, Error};
use std::path::Path;
use crate::libs::storage::{ List, Lists, Storage};

impl Lists {
    pub fn new(pth: String) -> Result<Lists> {
	Lists::load(pth)
    }
    fn to_str(&self) -> String {
	let str: String = self.lists
            .iter()
            .map(|list| {
		format!(
                    "[{}] {}",
                    list.id,
                    list.name
		)
            })
            .collect::<Vec<String>>()
            .join("\n");
	str
    }
    pub fn show(&self) {
	println!("[lists]---------------------------------------");
	println!("{}", self.to_str());
    }

    pub fn add(&mut self) -> Result<()> {
	println!("---------------------------------------");
	print!("enter list: ");
	stdout().flush().unwrap();
	let mut buffer = String::new();
	stdin().read_line(&mut buffer)?;
	let id = self.next_id;
	self.next_id += 1;
	if buffer.trim().to_string() == "" {
            println!("cancel");
	} else {
            self.lists.push(List {
		id: id,
		name: buffer.trim().to_string(),
            });
            let _ = self.save()?;
            println!("added {}", self.lists[self.lists.len() - 1].name);
	    self.show();
	}
	Ok(())
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
            self.lists.retain(|v| v.id != id);
            let _ = self.save()?;
            println!("{} deleted", id);
	    self.show();
	}
	Ok(())
    }
}

impl Storage<Lists> for Lists {
    fn save(&self) -> Result<()> {
	let path = Path::new(self.path.as_str());
	let mut file = File::create(&path)?;
	let j = serde_json::to_string(&self.lists)?;
	file.write_all(j.as_bytes())
    }
    fn load(pth: String) -> Result<Lists> {
	let path = Path::new(pth.as_str());
	let file = File::open(&path)?;
	let lists: Vec<List> = serde_json::from_reader(file)?;
	let mut next_id = 0u32;
	for l in lists.iter() {
	    if l.id > next_id { next_id = l.id };
	}
	Ok(Lists {
	    lists: lists,
	    path: pth,
	    next_id: next_id + 1
	})
    }
}
