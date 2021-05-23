use anyhow::Result;

use crate::core::{ AddURLReq, RemoveURLReq };

pub type TURLRepository = Box<dyn IURLRepository + 'static + Send + Sync>;

pub trait IURLRepository: Send + Sync {
    fn add(&self, req: AddURLReq) -> Result<String>;
    fn remove(&self, req: RemoveURLReq) -> Result<()>;
    fn get_redirection_url(&self, key: String) -> Result<String>;
}
