// Network configuration DTOs

use serde::{Deserialize, Serialize};
use crate::domain::network_entities::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WifiConfigDto {
    pub id: String,
    pub ssid: String,
    pub security_type: WifiSecurityType,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StaticIpConfigDto {
    pub id: String,
    pub interface_name: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: Option<String>,
    pub is_enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterfaceDto {
    pub name: String,
    pub interface_type: InterfaceType,
    pub mac_address: String,
    pub is_up: bool,
    pub current_ip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateWifiConfigRequest {
    pub ssid: String,
    pub password: String,
    pub security_type: WifiSecurityType,
}

#[derive(Debug, Deserialize)]
pub struct CreateStaticIpConfigRequest {
    pub interface_name: String,
    pub ip_address: String,
    pub subnet_mask: String,
    pub gateway: String,
    pub dns_primary: String,
    pub dns_secondary: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WifiConfigResponse {
    pub config: WifiConfigDto,
}

#[derive(Debug, Serialize)]
pub struct WifiConfigsListResponse {
    pub configs: Vec<WifiConfigDto>,
}

#[derive(Debug, Serialize)]
pub struct StaticIpConfigResponse {
    pub config: StaticIpConfigDto,
}

#[derive(Debug, Serialize)]
pub struct StaticIpConfigsListResponse {
    pub configs: Vec<StaticIpConfigDto>,
}

#[derive(Debug, Serialize)]
pub struct NetworkInterfacesResponse {
    pub interfaces: Vec<NetworkInterfaceDto>,
}

#[derive(Debug, Serialize)]
pub struct NetworkSettingsPageData {
    pub wifi_configs: Vec<WifiConfigDto>,
    pub static_ip_configs: Vec<StaticIpConfigDto>,
    pub network_interfaces: Vec<NetworkInterfaceDto>,
    pub active_wifi: Option<WifiConfigDto>,
}

// Conversion implementations
impl From<WifiConfig> for WifiConfigDto {
    fn from(config: WifiConfig) -> Self {
        Self {
            id: config.id,
            ssid: config.ssid,
            security_type: config.security_type,
            is_active: config.is_active,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}

impl From<&WifiConfig> for WifiConfigDto {
    fn from(config: &WifiConfig) -> Self {
        Self {
            id: config.id.clone(),
            ssid: config.ssid.clone(),
            security_type: config.security_type.clone(),
            is_active: config.is_active,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}

impl From<StaticIpConfig> for StaticIpConfigDto {
    fn from(config: StaticIpConfig) -> Self {
        Self {
            id: config.id,
            interface_name: config.interface_name,
            ip_address: config.ip_address,
            subnet_mask: config.subnet_mask,
            gateway: config.gateway,
            dns_primary: config.dns_primary,
            dns_secondary: config.dns_secondary,
            is_enabled: config.is_enabled,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}

impl From<&StaticIpConfig> for StaticIpConfigDto {
    fn from(config: &StaticIpConfig) -> Self {
        Self {
            id: config.id.clone(),
            interface_name: config.interface_name.clone(),
            ip_address: config.ip_address.clone(),
            subnet_mask: config.subnet_mask.clone(),
            gateway: config.gateway.clone(),
            dns_primary: config.dns_primary.clone(),
            dns_secondary: config.dns_secondary.clone(),
            is_enabled: config.is_enabled,
            created_at: config.created_at.to_rfc3339(),
        }
    }
}

impl From<NetworkInterface> for NetworkInterfaceDto {
    fn from(interface: NetworkInterface) -> Self {
        Self {
            name: interface.name,
            interface_type: interface.interface_type,
            mac_address: interface.mac_address,
            is_up: interface.is_up,
            current_ip: interface.current_ip,
        }
    }
}

impl From<&NetworkInterface> for NetworkInterfaceDto {
    fn from(interface: &NetworkInterface) -> Self {
        Self {
            name: interface.name.clone(),
            interface_type: interface.interface_type.clone(),
            mac_address: interface.mac_address.clone(),
            is_up: interface.is_up,
            current_ip: interface.current_ip.clone(),
        }
    }
}