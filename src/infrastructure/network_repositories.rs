// Network repository implementations

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use network_interface::{NetworkInterface as SystemNetworkInterface, NetworkInterfaceConfig, Addr};
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

// Real network interface repository using system interfaces
pub struct SystemNetworkInterfaceRepository;

impl SystemNetworkInterfaceRepository {
    pub fn new() -> Self {
        Self
    }

    fn determine_interface_type(name: &str) -> InterfaceType {
        if name.starts_with("lo") {
            InterfaceType::Loopback
        } else if name.starts_with("wl") || name.starts_with("wifi") || name.starts_with("wlan") {
            InterfaceType::Wireless
        } else if name.starts_with("eth") || name.starts_with("en") {
            InterfaceType::Ethernet
        } else {
            InterfaceType::Other
        }
    }

    fn convert_system_interface(sys_interface: &SystemNetworkInterface) -> NetworkInterface {
        let mut ipv4_addresses = Vec::new();
        let mut ipv6_addresses = Vec::new();

        for addr in &sys_interface.addr {
            match addr {
                Addr::V4(v4_addr) => ipv4_addresses.push(v4_addr.ip.to_string()),
                Addr::V6(v6_addr) => ipv6_addresses.push(v6_addr.ip.to_string()),
            }
        }

        // Keep current_ip for backward compatibility (first available address)
        let current_ip = sys_interface.addr.first().map(|addr| {
            match addr {
                Addr::V4(v4_addr) => v4_addr.ip.to_string(),
                Addr::V6(v6_addr) => v6_addr.ip.to_string(),
            }
        });

        NetworkInterface {
            name: sys_interface.name.clone(),
            interface_type: Self::determine_interface_type(&sys_interface.name),
            mac_address: "N/A".to_string(), // network-interface crate doesn't provide MAC address directly
            is_up: !ipv4_addresses.is_empty() || !ipv6_addresses.is_empty(),
            ipv4_addresses,
            ipv6_addresses,
            current_ip,
        }
    }
}

impl Default for SystemNetworkInterfaceRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl NetworkInterfaceRepository for SystemNetworkInterfaceRepository {
    async fn get_interfaces(&self) -> Result<Vec<NetworkInterface>, String> {
        let system_interfaces = SystemNetworkInterface::show()
            .map_err(|e| format!("Failed to get network interfaces: {}", e))?;

        let mut interface_map = std::collections::HashMap::new();

        // Group addresses by interface name
        for sys_interface in system_interfaces {
            let entry = interface_map.entry(sys_interface.name.clone()).or_insert_with(|| {
                (Self::determine_interface_type(&sys_interface.name), Vec::new())
            });
            entry.1.extend(sys_interface.addr);
        }

        // Convert grouped interfaces to NetworkInterface structs
        let mut interfaces = Vec::new();
        for (name, (interface_type, addresses)) in interface_map {
            let mut ipv4_addresses = Vec::new();
            let mut ipv6_addresses = Vec::new();

            for addr in &addresses {
                match addr {
                    Addr::V4(v4_addr) => ipv4_addresses.push(v4_addr.ip.to_string()),
                    Addr::V6(v6_addr) => ipv6_addresses.push(v6_addr.ip.to_string()),
                }
            }

            let current_ip = addresses.first().map(|addr| {
                match addr {
                    Addr::V4(v4_addr) => v4_addr.ip.to_string(),
                    Addr::V6(v6_addr) => v6_addr.ip.to_string(),
                }
            });

            interfaces.push(NetworkInterface {
                name,
                interface_type,
                mac_address: "N/A".to_string(),
                is_up: !ipv4_addresses.is_empty() || !ipv6_addresses.is_empty(),
                ipv4_addresses,
                ipv6_addresses,
                current_ip,
            });
        }

        Ok(interfaces)
    }

    async fn get_interface_by_name(&self, name: &str) -> Result<Option<NetworkInterface>, String> {
        let interfaces = self.get_interfaces().await?;
        Ok(interfaces.into_iter().find(|i| i.name == name))
    }
}