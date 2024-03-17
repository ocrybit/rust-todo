use std::io::{ Result };

pub trait Storage<T> {
    fn save(&self) -> Result<()>;
    fn load(pth: String) -> Result<T>;
}

#[derive(Clone, Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub done: bool,
    pub done_at: i64,
    pub lists: Vec<String>
}

pub struct Todos {
    pub todos: Vec<Task>,
    pub path: String,
    pub next_id: u32
}

pub struct List {
    pub id: u32,
    pub name: String
}

pub struct Lists {
    pub lists: Vec<List>,
    pub path: String,
    pub next_id: u32
}
