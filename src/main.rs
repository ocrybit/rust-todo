mod libs;
use libs::{ utils };
use libs::storage::{ Todos, Lists };
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

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
    let str = get_input("enter command: ", "exit")?;
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

fn get_input(txt : &str, empty : &str) -> Result<String> {
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
