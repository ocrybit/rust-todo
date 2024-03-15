use std::io::{self, stdin, Result, BufRead, BufReader};
use std::path::Path;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::Error;

struct Task {
    id: u32,
    name: String,
}

type Todos = Vec<Task>;

fn create_dir() -> Result<()> {
    let path = Path::new(".todos");
    fs::create_dir_all(path)?;
    Ok(())
}

fn to_str(todos: & Todos) -> String {
    todos.iter()
	.map(|task| format!("[{}] {}", task.id, task.name))
	.collect::<Vec<String>>().join("\n")
}

fn save(todos: & Todos) -> Result<()>{
    let path = Path::new(".todos/todos.txt");
    println!("{}",path.display());
    let mut file = File::create(&path)?;
    let todos_str = todos.iter()
	.map(|task| format!("{},{}", task.id, task.name))
	.collect::<Vec<String>>().join("\n");
    file.write_all(todos_str.as_bytes())
}

fn load() -> Result<Todos>{
    let path = Path::new(".todos/todos.txt");
    let file = File::open(&path)?;
    let buf_reader = BufReader::new(file);
    let mut tasks = Vec::new();
    for line in buf_reader.lines() {
	let line = line?;
	let parts: Vec<&str> = line.split(",").collect();
	let id = parts[0].parse::<u32>().map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
	tasks.push(Task {
	    id: id,
	    name: parts[1].to_string()
	});
    }
    Ok(tasks)
}

fn show (todos: &mut Todos){
    println!("---------------------------------------");
    println!("{}", to_str(todos));
    println!("---------------------------------------");
}

fn add (todos: &mut Todos) -> Result<()>{
    println!("---------------------------------------");
    println!("enter task({}):", todos.len());
    println!("---------------------------------------");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let id = todos.last().map_or(1, |task| task.id + 1);
    todos.push(Task { id: id, name: buffer.trim().to_string() });
    let _ = save(todos)?;
    println!("added {}", todos[todos.len() - 1].name);
    Ok(())
}

fn del (todos: &mut Todos) -> Result<()>{
    println!("---------------------------------------");
    println!("enter id({}):", todos.len());
    println!("---------------------------------------");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let id = buffer.trim().parse::<u32>().map_err(|e| Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    todos.retain(|v| v.id != id);
    let _ = save(todos)?;
    println!("{} deleted",id);
    Ok(())
}

fn exec(todos: &mut Todos, cmd : &str, prev: &mut String) -> Result<()>{
    match cmd {
	"1" | "show" | "s" => {
	    show(todos);
	    *prev = cmd.to_string();
	    return command(todos, prev)
	}
	"2" | "add" | "a" => {
	    let _ = add(todos);
	    *prev = cmd.to_string();
	    return command(todos, prev)
	}
	"3" | "del" | "d" => {
	    let _ = del(todos);
	    *prev = cmd.to_string();
	    return command(todos, prev)
	}
	"4" | "exit" | "e" => {
	    println!("bye!");
	}
	"0" | "" => {
	    return exec(todos, &prev.clone(), prev);
	}
	_ => {
	    println!("command not found...{}, {}", cmd, prev);
	    return command(todos, prev)
	}
    }
    Ok(())
}

fn command(todos: &mut Todos, prev: &mut String) -> Result<()>{
    println!("---------------------------------------");
    println!("enter command: 1) show, 2) add, 3) del, 4) exit");
    println!("---------------------------------------");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let cmd = buffer.as_str().trim();
    exec(todos, cmd, prev)
}

fn main() -> Result<()> {
    create_dir()?;
    let mut todos = load().unwrap_or_else(|_| vec![]);
    let mut prev = "1".to_string();
    command(&mut todos, &mut prev)
}
