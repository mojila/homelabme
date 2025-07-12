// Domain services - contain business logic and use cases

use async_trait::async_trait;
use crate::domain::entities::Greeting;
use crate::domain::repositories::GreetingRepository;
use std::sync::Arc;

#[async_trait]
pub trait GreetingService: Send + Sync {
    async fn create_greeting(&self, message: String, language: String) -> Result<Greeting, String>;
    async fn get_greeting(&self, id: &str) -> Result<Option<Greeting>, String>;
    async fn get_default_greeting(&self) -> Result<Greeting, String>;
    async fn list_greetings(&self) -> Result<Vec<Greeting>, String>;
}

pub struct GreetingServiceImpl {
    repository: Arc<dyn GreetingRepository>,
}

impl GreetingServiceImpl {
    pub fn new(repository: Arc<dyn GreetingRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl GreetingService for GreetingServiceImpl {
    async fn create_greeting(&self, message: String, language: String) -> Result<Greeting, String> {
        let greeting = Greeting::new(message, language);
        self.repository.save(&greeting).await?;
        Ok(greeting)
    }

    async fn get_greeting(&self, id: &str) -> Result<Option<Greeting>, String> {
        self.repository.find_by_id(id).await
    }

    async fn get_default_greeting(&self) -> Result<Greeting, String> {
        Ok(Greeting::default_hello_world())
    }

    async fn list_greetings(&self) -> Result<Vec<Greeting>, String> {
        self.repository.find_all().await
    }
}