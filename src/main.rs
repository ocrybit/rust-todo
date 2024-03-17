use std::io::prelude::*;
use std::io::{ stdout, stdin, Result };
mod libs;
use libs::{ utils };
use libs::storage::{ Todos, Lists };

fn exec(todos: &mut Todos, cmd: &str, prev: &mut String, lists: &mut Lists) -> Result<()> {
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    match parts[0] {
        "list-show" | "ls" => {
            lists.show();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "list-add" | "la" => {
            let _ = lists.add();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "list-del" | "ld" => {
            let _ = lists.del();
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
            let _ = todos.list();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "unlist" | "u" => {
            let _ = todos.unlist();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "help" | "h" => {
            todos.help();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "add" | "a" => {
            let _ = todos.add();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "complete" | "c" => {
            let _ = todos.complete();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "reorder" | "r" => {
            let _ = todos.reorder();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "trash" | "t" => {
            let _ = todos.trash();
            *prev = cmd.to_string();
            return command(todos, prev, lists);
        }
        "del" | "d" => {
            let _ = todos.del();
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
    print!("enter command: ");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let cmd = buffer.as_str().trim();
    exec(todos, cmd, prev, lists)
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
