mod libs;
use libs::{ utils };
use libs::storage::{ Todos, Lists };
use rustyline::{Result};

fn exec(todos: &mut Todos, cmd: &str, prev: &mut String, lists: &mut Lists) -> Result<()> {
    let mut parts: Vec<&str> = vec![""];
    if cmd != "" {
	parts = cmd.trim().split_whitespace().collect();
    }
    match parts[0] {
        "list-show" | "ls" => {
            lists.show();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "list-add" | "la" => {
	    if parts.len() == 1 {
		let _ = lists.add("");
	    }else{
		let _ = lists.add(parts[1]);
	    }
	    
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "list-del" | "ld" => {
	    if parts.len() == 1 {
		let _ = lists.del("");
	    }else{
		let _ = lists.del(parts[1]);
	    }
	    
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "show" | "s" => {
	    if parts.len() == 1 {
		todos.show("");
	    }else{
		todos.show(parts[1]);
	    }
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "list" | "l" => {
	    if parts.len() == 1 {
		let _ = todos.list("", "");
	    }else if parts.len() == 2 {
		let _ = todos.list(parts[1], "");
	    } else {
		let _ = todos.list(parts[1], parts[2]);
	    }
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "unlist" | "u" => {
	    if parts.len() == 1 {
		let _ = todos.unlist("", "");
	    }else if parts.len() == 2 {
		let _ = todos.unlist(parts[1], "");
	    } else {
		let _ = todos.unlist(parts[1], parts[2]);
	    }
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "help" | "h" => {
            todos.help();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "add" | "a" => {
	    if parts.len() == 1 {
		let _ = todos.add("");
	    }else{
		let _ = todos.add(parts[1]);
	    }
	    
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "complete" | "c" => {
	    if parts.len() == 1 {
		let _ = todos.complete("");
	    }else{
		let _ = todos.complete(parts[1]);
	    }
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "reorder" | "r" => {	
	    if parts.len() == 1 {
		let _ = todos.reorder("", "");
	    }else if parts.len() == 2 {
		let _ = todos.reorder(parts[1], "");
	    } else {
		let _ = todos.reorder(parts[1], parts[2]);
	    }    
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "trash" | "t" => {
            let _ = todos.trash();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "del" | "d" => {
	    if parts.len() == 1 {
		let _ = todos.del("");
	    }else{
		let _ = todos.del(parts[1]);
	    }
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "exit" | "e" => {
            println!("bye!");
        }
        "" => {
            return exec(todos, &prev.clone(), prev, lists);
        }
        _ => {
            println!("command not found...{}, {}", cmd, prev);
            return command(todos, prev, lists);
        }
    }
    Ok(())
}

fn command(todos: &mut Todos, prev: &mut String, lists: &mut Lists) -> Result<()> {
    println!("---------------------------------------");
    let str = utils::get_input("enter command: ", "exit")?;
    let cmd = str.trim();
    exec(todos, &cmd, prev, lists)
}

fn main() -> Result<()> {
    utils::create_dir()?;
    let mut todos = Todos::new(".todos/todos.txt".to_string())?;
    let mut lists = Lists::new(".todos/lists.txt".to_string())?;
    let mut prev = "1".to_string();
    lists.show();
    todos.show("");
    command(&mut todos, &mut prev, &mut lists)
}

