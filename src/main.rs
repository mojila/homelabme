// Clean Architecture modules
mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;
use infrastructure::repositories::InMemoryGreetingRepository;
use domain::services::GreetingServiceImpl;
use application::use_cases::*;
use infrastructure::web::{create_router, AppState};

#[tokio::main]
async fn main() {
    // Dependency injection - build the application from the outside in
    
    // Infrastructure layer
    let greeting_repository = Arc::new(InMemoryGreetingRepository::new());
    
    // Domain layer
    let greeting_service = Arc::new(GreetingServiceImpl::new(greeting_repository));
    
    // Application layer - use cases
    let get_default_greeting_use_case = Arc::new(GetDefaultGreetingUseCaseImpl::new(greeting_service.clone()));
    let create_greeting_use_case = Arc::new(CreateGreetingUseCaseImpl::new(greeting_service.clone()));
    let list_greetings_use_case = Arc::new(ListGreetingsUseCaseImpl::new(greeting_service));
    
    // Application state
    let app_state = AppState {
        get_default_greeting_use_case,
        create_greeting_use_case,
        list_greetings_use_case,
    };
    
    // Presentation layer - web routes
    let app = create_router(app_state);
    
    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    println!("📋 API Endpoints:");
    println!("   GET  /                     - HTML page");
    println!("   GET  /api/greetings/default - Get default greeting");
    println!("   GET  /api/greetings        - List all greetings");
    println!("   POST /api/greetings        - Create new greeting");
    
    axum::serve(listener, app).await.unwrap();
}
