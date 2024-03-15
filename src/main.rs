use std::io::{stdin, Result, BufRead, BufReader};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
    
struct Task {
    name: String,
}

fn save(todos: & Vec<Task>) -> Result<()>{
    let path = Path::new(".todos/todos.txt");
    println!("{}",path.display());
    let mut file = File::create(&path)?;
    let todos_str = todos.iter()
	.map(|task| format!("{}", task.name))
	.collect::<Vec<String>>().join("\n");
    file.write_all(todos_str.as_bytes())
}

fn load() -> Result<Vec<Task>>{
    let path = Path::new(".todos/todos.txt");
    let file = File::open(&path)?;
    let buf_reader = BufReader::new(file);
    let mut tasks = Vec::new();
    for  line in buf_reader.lines() {
	let name = line?;
	tasks.push(Task {
	    name
	});
    }
    Ok(tasks)
}

fn add (todos: &mut Vec<Task>) -> Result<()>{
    println!("enter task({}):", todos.len());
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    todos.push(Task { name: buffer.trim().to_string() });
    let _ = save(todos)?;
    println!("added {}", todos[todos.len() - 1].name);
    Ok(())
}

fn command(todos: &mut Vec<Task>) -> Result<()>{
    println!("enter command: 1) add, 2) done");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match buffer.as_str().trim() {
	"1" | "add" => {
	    let res = add(todos);
	    println!("{:?}", res);
	    return command(todos)
	}
	"2" | "done" => {
	    println!("bye!");
	}
	_ => {
	    println!("command not found...{}", buffer.trim());
	    return command(todos)
	}
    }
    Ok(())
}

fn main() -> Result<()> {
    let mut todos = load()?;
    command(&mut todos)
}
