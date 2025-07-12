// Network services - contain network configuration business logic

use async_trait::async_trait;
use std::sync::Arc;
use crate::domain::network_entities::*;
use crate::domain::network_repositories::*;

#[async_trait]
pub trait NetworkConfigService: Send + Sync {
    async fn create_wifi_config(&self, ssid: String, password: String, security_type: WifiSecurityType) -> Result<WifiConfig, String>;
    async fn get_wifi_configs(&self) -> Result<Vec<WifiConfig>, String>;
    async fn get_active_wifi_config(&self) -> Result<Option<WifiConfig>, String>;
    async fn activate_wifi_config(&self, id: &str) -> Result<(), String>;
    async fn delete_wifi_config(&self, id: &str) -> Result<(), String>;
    
    async fn create_static_ip_config(
        &self,
        interface_name: String,
        ip_address: String,
        subnet_mask: String,
        gateway: String,
        dns_primary: String,
        dns_secondary: Option<String>,
    ) -> Result<StaticIpConfig, String>;
    async fn get_static_ip_configs(&self) -> Result<Vec<StaticIpConfig>, String>;
    async fn get_static_ip_by_interface(&self, interface_name: &str) -> Result<Option<StaticIpConfig>, String>;
    async fn enable_static_ip(&self, id: &str) -> Result<(), String>;
    async fn disable_static_ip(&self, id: &str) -> Result<(), String>;
    async fn delete_static_ip_config(&self, id: &str) -> Result<(), String>;
    
    async fn get_network_interfaces(&self) -> Result<Vec<NetworkInterface>, String>;
    async fn apply_wifi_config(&self, config: &WifiConfig) -> Result<(), String>;
    async fn apply_static_ip_config(&self, config: &StaticIpConfig) -> Result<(), String>;
}

pub struct NetworkConfigServiceImpl {
    wifi_repository: Arc<dyn WifiConfigRepository>,
    static_ip_repository: Arc<dyn StaticIpConfigRepository>,
    interface_repository: Arc<dyn NetworkInterfaceRepository>,
}

impl NetworkConfigServiceImpl {
    pub fn new(
        wifi_repository: Arc<dyn WifiConfigRepository>,
        static_ip_repository: Arc<dyn StaticIpConfigRepository>,
        interface_repository: Arc<dyn NetworkInterfaceRepository>,
    ) -> Self {
        Self {
            wifi_repository,
            static_ip_repository,
            interface_repository,
        }
    }
}

#[async_trait]
impl NetworkConfigService for NetworkConfigServiceImpl {
    async fn create_wifi_config(&self, ssid: String, password: String, security_type: WifiSecurityType) -> Result<WifiConfig, String> {
        let config = WifiConfig::new(ssid, password, security_type);
        self.wifi_repository.save(&config).await?;
        Ok(config)
    }

    async fn get_wifi_configs(&self) -> Result<Vec<WifiConfig>, String> {
        self.wifi_repository.find_all().await
    }

    async fn get_active_wifi_config(&self) -> Result<Option<WifiConfig>, String> {
        self.wifi_repository.find_active().await
    }

    async fn activate_wifi_config(&self, id: &str) -> Result<(), String> {
        self.wifi_repository.set_active(id).await
    }

    async fn delete_wifi_config(&self, id: &str) -> Result<(), String> {
        self.wifi_repository.delete(id).await
    }

    async fn create_static_ip_config(
        &self,
        interface_name: String,
        ip_address: String,
        subnet_mask: String,
        gateway: String,
        dns_primary: String,
        dns_secondary: Option<String>,
    ) -> Result<StaticIpConfig, String> {
        let config = StaticIpConfig::new(
            interface_name,
            ip_address,
            subnet_mask,
            gateway,
            dns_primary,
            dns_secondary,
        );
        self.static_ip_repository.save(&config).await?;
        Ok(config)
    }

    async fn get_static_ip_configs(&self) -> Result<Vec<StaticIpConfig>, String> {
        self.static_ip_repository.find_all().await
    }

    async fn get_static_ip_by_interface(&self, interface_name: &str) -> Result<Option<StaticIpConfig>, String> {
        self.static_ip_repository.find_by_interface(interface_name).await
    }

    async fn enable_static_ip(&self, id: &str) -> Result<(), String> {
        self.static_ip_repository.enable(id).await
    }

    async fn disable_static_ip(&self, id: &str) -> Result<(), String> {
        self.static_ip_repository.disable(id).await
    }

    async fn delete_static_ip_config(&self, id: &str) -> Result<(), String> {
        self.static_ip_repository.delete(id).await
    }

    async fn get_network_interfaces(&self) -> Result<Vec<NetworkInterface>, String> {
        self.interface_repository.get_interfaces().await
    }

    async fn apply_wifi_config(&self, config: &WifiConfig) -> Result<(), String> {
        // In a real implementation, this would use system commands to configure WiFi
        // For now, we'll just simulate the operation
        println!("Applying WiFi config: SSID={}, Security={:?}", config.ssid, config.security_type);
        Ok(())
    }

    async fn apply_static_ip_config(&self, config: &StaticIpConfig) -> Result<(), String> {
        // In a real implementation, this would use system commands to configure static IP
        // For now, we'll just simulate the operation
        println!("Applying static IP config: Interface={}, IP={}", config.interface_name, config.ip_address);
        Ok(())
    }
}