// Network repository traits - define contracts for network configuration data access

use async_trait::async_trait;
use crate::domain::network_entities::*;

#[async_trait]
pub trait WifiConfigRepository: Send + Sync {
    async fn save(&self, config: &WifiConfig) -> Result<(), String>;
    async fn find_all(&self) -> Result<Vec<WifiConfig>, String>;
    async fn find_active(&self) -> Result<Option<WifiConfig>, String>;
    async fn set_active(&self, id: &str) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

#[async_trait]
pub trait StaticIpConfigRepository: Send + Sync {
    async fn save(&self, config: &StaticIpConfig) -> Result<(), String>;
    async fn find_all(&self) -> Result<Vec<StaticIpConfig>, String>;
    async fn enable(&self, id: &str) -> Result<(), String>;
    async fn disable(&self, id: &str) -> Result<(), String>;
    async fn delete(&self, id: &str) -> Result<(), String>;
}

#[async_trait]
pub trait NetworkInterfaceRepository: Send + Sync {
    async fn get_interfaces(&self) -> Result<Vec<NetworkInterface>, String>;
}