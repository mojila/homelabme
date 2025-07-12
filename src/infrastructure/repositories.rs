// Repository implementations - concrete data access implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::entities::Greeting;
use crate::domain::repositories::GreetingRepository;

// In-memory repository implementation
pub struct InMemoryGreetingRepository {
    storage: Arc<RwLock<HashMap<String, Greeting>>>,
}

impl InMemoryGreetingRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryGreetingRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GreetingRepository for InMemoryGreetingRepository {
    async fn save(&self, greeting: &Greeting) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(greeting.id.clone(), greeting.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Greeting>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Greeting>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }
}