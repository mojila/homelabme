// Network configuration entities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiConfig {
    pub id: String,
    pub ssid: String,
    pub password: String,
    pub security_type: WifiSecurityType,
    pub is_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WifiSecurityType {
    Open,
    WEP,
    WPA,
    WPA2,
    WPA3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticIpConfig {
    pub id: String,
    pub interface_name: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: Option<String>,
    pub is_enabled: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub interface_type: InterfaceType,
    pub mac_address: String,
    pub is_up: bool,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub current_ip: Option<String>, // Keep for backward compatibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterfaceType {
    Ethernet,
    Wireless,
    Loopback,
    Other,
}

impl WifiConfig {
    pub fn new(ssid: String, password: String, security_type: WifiSecurityType) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            ssid,
            password,
            security_type,
            is_active: false,
            created_at: chrono::Utc::now(),
        }
    }
}

impl StaticIpConfig {
    pub fn new(
        interface_name: String,
        ip_address: String,
        subnet_mask: String,
        gateway: String,
        dns_primary: String,
        dns_secondary: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            interface_name,
            ip_address,
            subnet_mask,
            gateway,
            dns_primary,
            dns_secondary,
            is_enabled: false,
            created_at: chrono::Utc::now(),
        }
    }
}