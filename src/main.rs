use std::io::{stdin, Result};

fn command() -> Result<()>{
    println!("enter command: 1) done");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    match buffer.as_str().trim_end() {
	"1" | "done" => {
	    println!("bye!");
	}
	_ => {
	    println!("command not found...{}", buffer.trim_end());
	    return command()
	}
    }
    Ok(())
}

fn main() -> Result<()> {
    command()
}
