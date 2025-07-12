// Repository traits - define contracts for data access
// These are interfaces that will be implemented in the infrastructure layer

use async_trait::async_trait;
use crate::domain::entities::Greeting;

#[async_trait]
pub trait GreetingRepository: Send + Sync {
    async fn save(&self, greeting: &Greeting) -> Result<(), String>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Greeting>, String>;
    async fn find_all(&self) -> Result<Vec<Greeting>, String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}