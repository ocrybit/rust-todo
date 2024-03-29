use std::path::Path;
use std::io::{Result};
use std::fs::{self};
use chrono::prelude::*;
use crate::libs::storage::{ Task };

pub fn create_dir() -> Result<()> {
    let path = Path::new(".todos");
    fs::create_dir_all(path)?;
    Ok(())
}

pub fn to_str(mut todos: Vec<Task>, tag: &str) -> String {
    if tag != "" {
	todos.retain(|v| v.lists.contains(&tag.to_string()));
    }
    let (mut dones, undones): (Vec<Task>, Vec<Task>) = todos.into_iter().partition(|v| v.done);
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
