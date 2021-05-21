use anyhow::Result;

pub trait IURLRepository {
    fn add() -> Result<String>;
    fn remove() -> Result<()>;
}

pub struct URLRepository {
    pub db: String
}

impl URLRepository {
    pub fn new() -> Self {
        URLRepository {
            db: "".to_string(),
        }
    }
}

impl IURLRepository for URLRepository {
    fn add() -> Result<String> {
        return Ok(String::new());
    }

    fn remove() -> Result<()> {
        return Ok(());
    }
}
