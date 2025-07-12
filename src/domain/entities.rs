// Domain entities - core business objects

#[derive(Debug, Clone)]
pub struct Greeting {
    pub id: String,
    pub message: String,
    pub language: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Greeting {
    pub fn new(message: String, language: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            message,
            language,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn default_hello_world() -> Self {
        Self::new(
            "Hello, World!".to_string(),
            "en".to_string(),
        )
    }
}