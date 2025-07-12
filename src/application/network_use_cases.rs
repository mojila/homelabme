// Network configuration use cases

use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::network_services::NetworkConfigService;
use crate::application::network_dto::*;

#[async_trait]
pub trait GetNetworkSettingsUseCase: Send + Sync {
    async fn execute(&self) -> Result<NetworkSettingsPageData, String>;
}

#[async_trait]
pub trait CreateWifiConfigUseCase: Send + Sync {
    async fn execute(&self, request: CreateWifiConfigRequest) -> Result<WifiConfigResponse, String>;
}

#[async_trait]
pub trait ActivateWifiConfigUseCase: Send + Sync {
    async fn execute(&self, config_id: String) -> Result<(), String>;
}

#[async_trait]
pub trait DeleteWifiConfigUseCase: Send + Sync {
    async fn execute(&self, config_id: String) -> Result<(), String>;
}

#[async_trait]
pub trait CreateStaticIpConfigUseCase: Send + Sync {
    async fn execute(&self, request: CreateStaticIpConfigRequest) -> Result<StaticIpConfigResponse, String>;
}

#[async_trait]
pub trait EnableStaticIpConfigUseCase: Send + Sync {
    async fn execute(&self, config_id: String) -> Result<(), String>;
}

#[async_trait]
pub trait DisableStaticIpConfigUseCase: Send + Sync {
    async fn execute(&self, config_id: String) -> Result<(), String>;
}

#[async_trait]
pub trait DeleteStaticIpConfigUseCase: Send + Sync {
    async fn execute(&self, config_id: String) -> Result<(), String>;
}

#[async_trait]
pub trait ScanWifiNetworksUseCase: Send + Sync {
    async fn execute(&self) -> Result<Vec<ScannedWifiNetworkDto>, String>;
}

// Implementations
pub struct GetNetworkSettingsUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl GetNetworkSettingsUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl GetNetworkSettingsUseCase for GetNetworkSettingsUseCaseImpl {
    async fn execute(&self) -> Result<NetworkSettingsPageData, String> {
        let wifi_configs = self.network_service.get_wifi_configs().await?
            .into_iter().map(|c| c.into()).collect();
        
        let static_ip_configs = self.network_service.get_static_ip_configs().await?
            .into_iter().map(|c| c.into()).collect();
        
        let network_interfaces = self.network_service.get_network_interfaces().await?
            .into_iter().map(|i| i.into()).collect();
        
        let active_wifi = self.network_service.get_active_wifi_config().await?
            .map(|c| c.into());
        
        Ok(NetworkSettingsPageData {
            wifi_configs,
            static_ip_configs,
            network_interfaces,
            active_wifi,
        })
    }
}

pub struct CreateWifiConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl CreateWifiConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl CreateWifiConfigUseCase for CreateWifiConfigUseCaseImpl {
    async fn execute(&self, request: CreateWifiConfigRequest) -> Result<WifiConfigResponse, String> {
        let config = self.network_service.create_wifi_config(
            request.ssid,
            request.password,
            request.security_type,
        ).await?;
        
        Ok(WifiConfigResponse {
            config: config.into(),
        })
    }
}

pub struct ActivateWifiConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl ActivateWifiConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl ActivateWifiConfigUseCase for ActivateWifiConfigUseCaseImpl {
    async fn execute(&self, config_id: String) -> Result<(), String> {
        self.network_service.activate_wifi_config(&config_id).await
    }
}

pub struct DeleteWifiConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl DeleteWifiConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl DeleteWifiConfigUseCase for DeleteWifiConfigUseCaseImpl {
    async fn execute(&self, config_id: String) -> Result<(), String> {
        self.network_service.delete_wifi_config(&config_id).await
    }
}

pub struct CreateStaticIpConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl CreateStaticIpConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl CreateStaticIpConfigUseCase for CreateStaticIpConfigUseCaseImpl {
    async fn execute(&self, request: CreateStaticIpConfigRequest) -> Result<StaticIpConfigResponse, String> {
        let config = self.network_service.create_static_ip_config(
            request.interface_name,
            request.ip_address,
            request.subnet_mask,
            request.gateway,
            request.dns_primary,
            request.dns_secondary,
        ).await?;
        
        Ok(StaticIpConfigResponse {
            config: config.into(),
        })
    }
}

pub struct EnableStaticIpConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl EnableStaticIpConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl EnableStaticIpConfigUseCase for EnableStaticIpConfigUseCaseImpl {
    async fn execute(&self, config_id: String) -> Result<(), String> {
        self.network_service.enable_static_ip(&config_id).await
    }
}

pub struct DisableStaticIpConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl DisableStaticIpConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl DisableStaticIpConfigUseCase for DisableStaticIpConfigUseCaseImpl {
    async fn execute(&self, config_id: String) -> Result<(), String> {
        self.network_service.disable_static_ip(&config_id).await
    }
}

pub struct DeleteStaticIpConfigUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl DeleteStaticIpConfigUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl DeleteStaticIpConfigUseCase for DeleteStaticIpConfigUseCaseImpl {
    async fn execute(&self, config_id: String) -> Result<(), String> {
        self.network_service.delete_static_ip_config(&config_id).await
    }
}

pub struct ScanWifiNetworksUseCaseImpl {
    network_service: Arc<dyn NetworkConfigService>,
}

impl ScanWifiNetworksUseCaseImpl {
    pub fn new(network_service: Arc<dyn NetworkConfigService>) -> Self {
        Self { network_service }
    }
}

#[async_trait]
impl ScanWifiNetworksUseCase for ScanWifiNetworksUseCaseImpl {
    async fn execute(&self) -> Result<Vec<ScannedWifiNetworkDto>, String> {
        let networks = self.network_service.scan_wifi_networks().await?;
        Ok(networks.into_iter().map(|n| n.into()).collect())
    }
}