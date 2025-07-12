// Clean Architecture modules
mod domain;
mod application;
mod infrastructure;

use std::sync::Arc;
use infrastructure::repositories::InMemoryGreetingRepository;
use domain::services::GreetingServiceImpl;
use application::use_cases::*;
use application::network_use_cases::*;
use application::network_dto::*;
use domain::network_services::*;
use domain::network_entities::*;
use domain::network_repositories::*;
use infrastructure::network_repositories::*;
use infrastructure::web::{create_router, AppState};

#[tokio::main]
async fn main() {
    // Dependency injection - build the application from the outside in
    
    // Infrastructure layer
    let greeting_repository = Arc::new(InMemoryGreetingRepository::new());
    let wifi_config_repository = Arc::new(InMemoryWifiConfigRepository::new());
    let static_ip_config_repository = Arc::new(InMemoryStaticIpConfigRepository::new());
    let network_interface_repository = Arc::new(SystemNetworkInterfaceRepository::new());
    
    // Domain layer
    let greeting_service = Arc::new(GreetingServiceImpl::new(greeting_repository));
    let network_config_service = Arc::new(NetworkConfigServiceImpl::new(
        wifi_config_repository.clone(),
        static_ip_config_repository.clone(),
        network_interface_repository.clone(),
    ));
    
    // Application layer - use cases
    let get_default_greeting_use_case = Arc::new(GetDefaultGreetingUseCaseImpl::new(greeting_service.clone()));
    let create_greeting_use_case = Arc::new(CreateGreetingUseCaseImpl::new(greeting_service.clone()));
    let list_greetings_use_case = Arc::new(ListGreetingsUseCaseImpl::new(greeting_service));
    
    // Network use cases
    let get_network_settings_use_case = Arc::new(GetNetworkSettingsUseCaseImpl::new(network_config_service.clone()));
    let create_wifi_config_use_case = Arc::new(CreateWifiConfigUseCaseImpl::new(network_config_service.clone()));
    let activate_wifi_config_use_case = Arc::new(ActivateWifiConfigUseCaseImpl::new(network_config_service.clone()));
    let delete_wifi_config_use_case = Arc::new(DeleteWifiConfigUseCaseImpl::new(network_config_service.clone()));
    let create_static_ip_config_use_case = Arc::new(CreateStaticIpConfigUseCaseImpl::new(network_config_service.clone()));
    let enable_static_ip_config_use_case = Arc::new(EnableStaticIpConfigUseCaseImpl::new(network_config_service.clone()));
    let disable_static_ip_config_use_case = Arc::new(DisableStaticIpConfigUseCaseImpl::new(network_config_service.clone()));
    let delete_static_ip_config_use_case = Arc::new(DeleteStaticIpConfigUseCaseImpl::new(network_config_service.clone()));
    let scan_wifi_networks_use_case = Arc::new(ScanWifiNetworksUseCaseImpl::new(network_config_service.clone()));
    
    // Application state
    let app_state = AppState {
        get_default_greeting_use_case,
        create_greeting_use_case,
        list_greetings_use_case,
        get_network_settings_use_case,
        create_wifi_config_use_case,
        activate_wifi_config_use_case,
        delete_wifi_config_use_case,
        create_static_ip_config_use_case,
        enable_static_ip_config_use_case,
        disable_static_ip_config_use_case,
        delete_static_ip_config_use_case,
        scan_wifi_networks_use_case,
    };
    
    // Presentation layer - web routes
    let app = create_router(app_state);
    
    // Start the server
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "80".to_string())
        .parse::<u16>()
        .unwrap_or(80);
    
    let bind_address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();
    
    let server_url = if port == 80 {
        "http://localhost".to_string()
    } else {
        format!("http://localhost:{}", port)
    };
    
    println!("🦀 Rust Clean Architecture Server running on {}", server_url);
    println!("📋 Available endpoints:");
    println!("   GET  /                     - Network settings page");
    println!("   GET  /api/greetings/default - Get default greeting");
    println!("   GET  /api/greetings        - List all greetings");
    println!("   POST /api/greetings        - Create new greeting");
    println!("   GET  /api/network/settings - Get network settings");
    println!("   POST /api/network/wifi     - Create WiFi config");
    println!("   POST /api/network/static-ip - Create static IP config");
    
    axum::serve(listener, app).await.unwrap();
}
