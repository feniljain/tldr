use anyhow::Result;
use crate::pkg::repository::repository::{IURLRepository, URLRepository};

pub trait IURLService {
     fn add() -> Result<String>;
     fn remove() -> Result<()>;
}

pub struct URLService<T: IURLRepository> {
    pub repo: T,
}

impl URLService<URLRepository> {
    pub fn new() -> Self {
        URLService {
            repo: URLRepository::new(),
        }
    }
}

impl IURLService for URLService<URLRepository> {
    fn add() -> Result<String> {
        return Ok(String::new());
    }

    fn remove() -> Result<()> {
        return Ok(());
    }
}
