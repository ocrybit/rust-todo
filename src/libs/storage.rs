use std::io::{ Result };

pub trait Storage<T> {
    fn save(&self) -> Result<()>;
    fn load(pth: String) -> Result<T>;
}
