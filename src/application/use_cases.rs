// Use cases - define specific application operations

use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::services::GreetingService;
use crate::application::dto::*;

#[async_trait]
pub trait GetDefaultGreetingUseCase: Send + Sync {
    async fn execute(&self) -> Result<GreetingResponse, String>;
}

#[async_trait]
pub trait CreateGreetingUseCase: Send + Sync {
    async fn execute(&self, request: CreateGreetingRequest) -> Result<GreetingResponse, String>;
}

#[async_trait]
pub trait ListGreetingsUseCase: Send + Sync {
    async fn execute(&self) -> Result<GreetingsListResponse, String>;
}

pub struct GetDefaultGreetingUseCaseImpl {
    greeting_service: Arc<dyn GreetingService>,
}

impl GetDefaultGreetingUseCaseImpl {
    pub fn new(greeting_service: Arc<dyn GreetingService>) -> Self {
        Self { greeting_service }
    }
}

#[async_trait]
impl GetDefaultGreetingUseCase for GetDefaultGreetingUseCaseImpl {
    async fn execute(&self) -> Result<GreetingResponse, String> {
        let greeting = self.greeting_service.get_default_greeting().await?;
        Ok(GreetingResponse {
            greeting: greeting.into(),
        })
    }
}

pub struct CreateGreetingUseCaseImpl {
    greeting_service: Arc<dyn GreetingService>,
}

impl CreateGreetingUseCaseImpl {
    pub fn new(greeting_service: Arc<dyn GreetingService>) -> Self {
        Self { greeting_service }
    }
}

#[async_trait]
impl CreateGreetingUseCase for CreateGreetingUseCaseImpl {
    async fn execute(&self, request: CreateGreetingRequest) -> Result<GreetingResponse, String> {
        let language = request.language.unwrap_or_else(|| "en".to_string());
        let greeting = self.greeting_service.create_greeting(request.message, language).await?;
        Ok(GreetingResponse {
            greeting: greeting.into(),
        })
    }
}

pub struct ListGreetingsUseCaseImpl {
    greeting_service: Arc<dyn GreetingService>,
}

impl ListGreetingsUseCaseImpl {
    pub fn new(greeting_service: Arc<dyn GreetingService>) -> Self {
        Self { greeting_service }
    }
}

#[async_trait]
impl ListGreetingsUseCase for ListGreetingsUseCaseImpl {
    async fn execute(&self) -> Result<GreetingsListResponse, String> {
        let greetings = self.greeting_service.list_greetings().await?;
        Ok(GreetingsListResponse {
            greetings: greetings.iter().map(|g| g.into()).collect(),
        })
    }
}