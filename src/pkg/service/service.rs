use anyhow::{ Result, anyhow };

use crate::pkg::repository::IURLRepository;
use crate::pkg::db::json::DBJson;
use crate::core::{ Config, AddURLReq, RemoveURLReq };

pub type TURLService = Box<dyn IURLService + 'static + Send + Sync>;

pub trait IURLService: Send + Sync {
    fn add(&self, req: AddURLReq) -> Result<String>;
    fn remove(&self, req: RemoveURLReq) -> Result<()>;
    fn get_redirection_url(&self, key: String) -> Result<String>;
}

pub struct URLService {
    pub repo: Box<dyn IURLRepository>,
}

impl URLService {
    pub fn new(config: Config) -> Result<TURLService> {
        match config.db_type.as_str() {
            "json" => Ok(Box::new(URLService {
                repo: DBJson::new(config),
            })),
            _ => Err(anyhow!("invalid db type")),
        }
    }
}

impl IURLService for URLService {
    fn add(&self, req: AddURLReq) -> Result<String> {
        self.repo.add(req)
    }

    fn remove(&self, req: RemoveURLReq) -> Result<()> {
        self.repo.remove(req)
    }

    fn get_redirection_url(&self, key: String) -> Result<String> {
        self.repo.get_redirection_url(key)
    }
}
