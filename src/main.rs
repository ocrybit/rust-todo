use std::io::prelude::*;
use std::io::{ stdout, stdin, Result };
mod utils;
mod list;
use list::{ List };

fn exec(list: &mut List, cmd: &str, prev: &mut String) -> Result<()> {
    match cmd {
        "show" | "s" => {
            list.show();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "help" | "h" => {
            list.help();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "add" | "a" => {
            let _ = list.add();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "complete" | "c" => {
            let _ = list.complete();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "reorder" | "r" => {
            let _ = list.reorder();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "trash" | "t" => {
            let _ = list.trash();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "del" | "d" => {
            let _ = list.del();
            *prev = cmd.to_string();
            return command(list, prev);
        }
        "exit" | "e" => {
            println!("bye!");
        }
        "" => {
            return exec(list, &prev.clone(), prev);
        }
        _ => {
            println!("command not found...{}, {}", cmd, prev);
            return command(list, prev);
        }
    }
    Ok(())
}

fn command(list: &mut List, prev: &mut String) -> Result<()> {
    println!("---------------------------------------");
    print!("enter command: ");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    let cmd = buffer.as_str().trim();
    exec(list, cmd, prev)
}

fn main() -> Result<()> {
    utils::create_dir()?;
    let mut list = List::new(".todos/todos.txt".to_string())?;
    let mut prev = "1".to_string();
    list.show();
    command(&mut list, &mut prev)
}
