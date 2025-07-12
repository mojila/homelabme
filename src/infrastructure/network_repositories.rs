// Network repository implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::network_entities::*;
use crate::domain::network_repositories::*;

// In-memory WiFi configuration repository
pub struct InMemoryWifiConfigRepository {
    storage: Arc<RwLock<HashMap<String, WifiConfig>>>,
}

impl InMemoryWifiConfigRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryWifiConfigRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl WifiConfigRepository for InMemoryWifiConfigRepository {
    async fn save(&self, config: &WifiConfig) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(config.id.clone(), config.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<WifiConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<WifiConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn find_active(&self) -> Result<Option<WifiConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().find(|config| config.is_active).cloned())
    }

    async fn set_active(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        
        // Deactivate all configs first
        for config in storage.values_mut() {
            config.is_active = false;
        }
        
        // Activate the specified config
        if let Some(config) = storage.get_mut(id) {
            config.is_active = true;
            Ok(())
        } else {
            Err("WiFi config not found".to_string())
        }
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }
}

// In-memory Static IP configuration repository
pub struct InMemoryStaticIpConfigRepository {
    storage: Arc<RwLock<HashMap<String, StaticIpConfig>>>,
}

impl InMemoryStaticIpConfigRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryStaticIpConfigRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl StaticIpConfigRepository for InMemoryStaticIpConfigRepository {
    async fn save(&self, config: &StaticIpConfig) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.insert(config.id.clone(), config.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<StaticIpConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.get(id).cloned())
    }

    async fn find_by_interface(&self, interface_name: &str) -> Result<Option<StaticIpConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().find(|config| config.interface_name == interface_name).cloned())
    }

    async fn find_all(&self) -> Result<Vec<StaticIpConfig>, String> {
        let storage = self.storage.read().await;
        Ok(storage.values().cloned().collect())
    }

    async fn enable(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        if let Some(config) = storage.get_mut(id) {
            config.is_enabled = true;
            Ok(())
        } else {
            Err("Static IP config not found".to_string())
        }
    }

    async fn disable(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        if let Some(config) = storage.get_mut(id) {
            config.is_enabled = false;
            Ok(())
        } else {
            Err("Static IP config not found".to_string())
        }
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().await;
        storage.remove(id);
        Ok(())
    }
}

// Mock network interface repository
pub struct MockNetworkInterfaceRepository;

impl MockNetworkInterfaceRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for MockNetworkInterfaceRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NetworkInterfaceRepository for MockNetworkInterfaceRepository {
    async fn get_interfaces(&self) -> Result<Vec<NetworkInterface>, String> {
        // Mock data for demonstration
        Ok(vec![
            NetworkInterface {
                name: "eth0".to_string(),
                interface_type: InterfaceType::Ethernet,
                mac_address: "00:1B:44:11:3A:B7".to_string(),
                is_up: true,
                current_ip: Some("192.168.1.100".to_string()),
            },
            NetworkInterface {
                name: "wlan0".to_string(),
                interface_type: InterfaceType::Wireless,
                mac_address: "00:1B:44:11:3A:B8".to_string(),
                is_up: true,
                current_ip: Some("192.168.1.101".to_string()),
            },
            NetworkInterface {
                name: "lo".to_string(),
                interface_type: InterfaceType::Loopback,
                mac_address: "00:00:00:00:00:00".to_string(),
                is_up: true,
                current_ip: Some("127.0.0.1".to_string()),
            },
        ])
    }

    async fn get_interface_by_name(&self, name: &str) -> Result<Option<NetworkInterface>, String> {
        let interfaces = self.get_interfaces().await?;
        Ok(interfaces.into_iter().find(|i| i.name == name))
    }
}