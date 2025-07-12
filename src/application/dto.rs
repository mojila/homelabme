// Data Transfer Objects - for data transformation between layers

use serde::{Deserialize, Serialize};
use crate::domain::entities::Greeting;

#[derive(Debug, Serialize, Deserialize)]
pub struct GreetingDto {
    pub id: String,
    pub message: String,
    pub language: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateGreetingRequest {
    pub message: String,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GreetingResponse {
    pub greeting: GreetingDto,
}

#[derive(Debug, Serialize)]
pub struct GreetingsListResponse {
    pub greetings: Vec<GreetingDto>,
}

impl From<Greeting> for GreetingDto {
    fn from(greeting: Greeting) -> Self {
        Self {
            id: greeting.id,
            message: greeting.message,
            language: greeting.language,
            created_at: greeting.created_at.to_rfc3339(),
        }
    }
}

impl From<&Greeting> for GreetingDto {
    fn from(greeting: &Greeting) -> Self {
        Self {
            id: greeting.id.clone(),
            message: greeting.message.clone(),
            language: greeting.language.clone(),
            created_at: greeting.created_at.to_rfc3339(),
        }
    }
}