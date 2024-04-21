# A Simple Todo App in Rust

## Compile and Run 

```bash
cargo make run
```

## Test

```bash
cargo test
```

## Generate Docs

```bash
cargo doc
```

## Commands

- `[empty]` : repeat the previous command
- `s | show` : show tasks
- `a | add` : add a task
- `e | edit` : edit a task
- `c | complete` : complete a task
- `m | move` : move a task
- `rr` | `reset` : reset recursive tasks
- `d | del` : delete a task
- `t | trash` : trash completed tasks
- `l | list` : add a task to a list
- `u | unlist` : remove a task from a list
- `ls | list-show` : show lists
- `la | list-add` : add a list
- `ld | list-del` : delete a list
- `h | help` : show available commands
- `q | quit` : quit the app

## Special Tag Types

- `#` : non task
- `@` : recursive
