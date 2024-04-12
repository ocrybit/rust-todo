use std::path::Path;
use std::fs::{self};
use chrono::prelude::*;
use crate::libs::storage::{ Task };
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn get_values<'a>(msg: &'a str, msg2: &'a str, empty: &'a str, _id: &'a str, _id2: &'a str) -> Result<(String, String)> {
    let mut __id = "";
    let mut __id2 = "";
    let buffer: String;
    if _id == "" && _id2 == "" {
	buffer = get_input(msg, empty)?;
	let parts: Vec<&str> = buffer.trim().split(",").collect();
	__id = parts[0].trim();
	__id2 = if parts.len() < 2 { "" } else {parts[1].trim()};
    } else if _id2 == "" {
	__id = _id;
	buffer = get_input(msg2, empty)?;
	__id2 = buffer.trim();
    }else{
	__id = _id;
	__id2 = _id2;
    }
    Ok((__id.to_string(), __id2.to_string()))
}

pub fn get_values3<'a>(msg: &'a str, msg2: &'a str, msg3: &'a str, empty: &'a str, _id: &'a str, _id2: &'a str, _id3: &'a str) -> Result<(String, String, String)> {
    let mut __id = "";
    let mut __id2 = "";
    let mut __id3 = "";
    
    let buffer: String;
    if _id == "" && _id2 == "" && _id3 == "" {
	buffer = get_input(msg, empty)?;	
	let parts: Vec<&str> = buffer.trim().split(",").collect();
	__id = parts[0].trim();
	__id2 = if parts.len() < 2 { "" } else {parts[1].trim()};
	__id3 = if parts.len() < 3 { "" } else {parts[2].trim()};
    }else if _id2 == "" && _id3 == "" {
	__id = _id;
	buffer = get_input(msg2, empty)?;
	let parts: Vec<&str> = buffer.trim().split(",").collect();
	__id2 = parts[0].trim();
	__id3 = if parts.len() < 2 { "" } else {parts[1].trim()};
    } else if _id3 == "" {
	__id = _id;
	__id2 = _id2;
	buffer = get_input(msg3, empty)?;
	__id3 = buffer.trim();
    }else{
	__id = _id;
	__id2 = _id2;
	__id3 = _id3;
    }
    Ok((__id.to_string(), __id2.to_string(), __id3.to_string()))
}

pub fn get_value(msg: &str, empty: &str, _id: &str) -> Result<String> {
    if _id == "" {
	Ok(get_input(msg, empty)?.trim().to_string())
    } else {
	Ok(_id.to_string())
    }
}

pub fn bar() {
    println!("--------------------------------------------");
}

pub fn bar2(name: &str) {
    println!("\n({}) ===================================", name);
}

pub fn create_dir() -> Result<()> {
    let path = Path::new(".todos");
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn to_str(mut todos: Vec<Task>, tag: &str) -> String {
    if tag == "_" {
	todos.retain(|v| v.lists.len() == 0 || v.lists.contains(&("_".to_string())));
    } else if tag != "" {
	todos.retain(|v| v.lists.contains(&tag.to_string()));
    } else {
	todos.retain(|todo| {
	    !todo.lists.iter().any(|list_item| list_item.starts_with("#") || list_item.starts_with("@"))
	});
    }
    
    let (mut dones, undones): (Vec<Task>, Vec<Task>) = todos.into_iter().partition(|v| v.done);
    let mut i = 0;
    let mut str: String = undones
        .iter()
        .map(|task| {
	    i += 1;
	    let mut lists = task.lists.iter().map(|l| format!("{}", l)).collect::<Vec<String>>().join("|");
	    lists = format!("< {} >",lists);
	    format!(
                "[ {:0>2} # {:0>3} ] {:<30} {}",
		i - 1,
                task.id,
                task.name,
		if task.lists.len() == 0 || (task.lists.len() == 1 && task.lists[0] == "") { "".to_string() } else { lists }
	    )
        })
        .collect::<Vec<String>>()
        .join("\n");
    if dones.len() > 0 {
        str += "\n-------------------------------------- [ done ]\n";
	dones.sort_by_key(|v| v.done_at);
        str += &(dones
		 .iter()
		 .map(|task| {
		     let ts = task.done_at / 1000;
		     let datetime: DateTime<Utc> = Utc.timestamp_opt(ts, 0).unwrap();
		     let ts2 = datetime.format("%m/%d").to_string();
		     format!(
			 "[ {:0>2} # {:0>3} ] {:<30} {}",
			 i,
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

pub fn get_input(txt : &str, empty : &str) -> Result<String> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history(".todos/history.txt").is_err() {
        println!("No previous history.");
    }
    let mut str = String::new();
    loop {
        let readline = rl.readline(txt);
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
		str = line;
		break
            },
            Err(ReadlineError::Interrupted) => {
		str = empty.to_string();
                break
            },
            Err(ReadlineError::Eof) => {
                break
            },
            Err(_) => {
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    let _ = rl.save_history(".todos/history.txt");
    Ok(str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str(){
	let task = Task { id: 1, name: "test".to_string(), done: false, done_at: 0, lists: vec!["dev".to_string()] };
	let todos = vec![task];
	assert_eq!(to_str(todos, ""), "[1] test <dev>");
    }
}
