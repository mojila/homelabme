// Web infrastructure - Axum handlers and routing

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::application::use_cases::*;
use crate::application::dto::*;

// Application state containing use cases
#[derive(Clone)]
pub struct AppState {
    pub get_default_greeting_use_case: Arc<dyn GetDefaultGreetingUseCase>,
    pub create_greeting_use_case: Arc<dyn CreateGreetingUseCase>,
    pub list_greetings_use_case: Arc<dyn ListGreetingsUseCase>,
}

// Create the router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world_handler))
        .route("/api/greetings", get(list_greetings_handler))
        .route("/api/greetings", post(create_greeting_handler))
        .route("/api/greetings/default", get(get_default_greeting_handler))
        .with_state(state)
}

// HTML page handler
async fn hello_world_handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    match state.get_default_greeting_use_case.execute().await {
        Ok(response) => {
            let greeting = &response.greeting;
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Hello World - Rust Clean Architecture</title>
                    <style>
                        body {{
                            font-family: Arial, sans-serif;
                            display: flex;
                            justify-content: center;
                            align-items: center;
                            height: 100vh;
                            margin: 0;
                            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                            color: white;
                        }}
                        .container {{
                            text-align: center;
                            padding: 2rem;
                            background: rgba(255, 255, 255, 0.1);
                            border-radius: 10px;
                            backdrop-filter: blur(10px);
                            box-shadow: 0 8px 32px 0 rgba(31, 38, 135, 0.37);
                        }}
                        h1 {{
                            font-size: 3rem;
                            margin-bottom: 1rem;
                            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
                        }}
                        p {{
                            font-size: 1.2rem;
                            margin-bottom: 0.5rem;
                        }}
                        .rust-logo {{
                            font-size: 4rem;
                            margin-bottom: 1rem;
                        }}
                        .api-info {{
                            margin-top: 2rem;
                            padding: 1rem;
                            background: rgba(255, 255, 255, 0.05);
                            border-radius: 5px;
                            font-size: 0.9rem;
                        }}
                        .api-endpoint {{
                            margin: 0.5rem 0;
                            font-family: monospace;
                            background: rgba(0, 0, 0, 0.2);
                            padding: 0.3rem;
                            border-radius: 3px;
                        }}
                    </style>
                </head>
                <body>
                    <div class="container">
                        <div class="rust-logo">🦀</div>
                        <h1>{}</h1>
                        <p>Welcome to your Rust web application</p>
                        <p>Built with Clean Architecture & Axum</p>
                        <p>Language: {} | ID: {}</p>
                        <div class="api-info">
                            <h3>Available API Endpoints:</h3>
                            <div class="api-endpoint">GET /api/greetings/default</div>
                            <div class="api-endpoint">GET /api/greetings</div>
                            <div class="api-endpoint">POST /api/greetings</div>
                        </div>
                    </div>
                </body>
                </html>
                "#,
                greeting.message, greeting.language, greeting.id
            );
            Ok(Html(html))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// API handlers
async fn get_default_greeting_handler(
    State(state): State<AppState>,
) -> Result<Json<GreetingResponse>, StatusCode> {
    match state.get_default_greeting_use_case.execute().await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_greeting_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateGreetingRequest>,
) -> Result<Json<GreetingResponse>, StatusCode> {
    match state.create_greeting_use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_greetings_handler(
    State(state): State<AppState>,
) -> Result<Json<GreetingsListResponse>, StatusCode> {
    match state.list_greetings_use_case.execute().await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}