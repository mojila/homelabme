// Web infrastructure - Axum handlers and routing

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post, delete},
    Router,
};
use std::sync::Arc;
use crate::application::use_cases::*;
use crate::application::dto::*;
use crate::application::network_use_cases::*;
use crate::application::network_dto::*;

// Application state containing use cases
#[derive(Clone)]
pub struct AppState {
    pub get_default_greeting_use_case: Arc<dyn GetDefaultGreetingUseCase>,
    pub create_greeting_use_case: Arc<dyn CreateGreetingUseCase>,
    pub list_greetings_use_case: Arc<dyn ListGreetingsUseCase>,
    // Network use cases
    pub get_network_settings_use_case: Arc<dyn GetNetworkSettingsUseCase>,
    pub create_wifi_config_use_case: Arc<dyn CreateWifiConfigUseCase>,
    pub activate_wifi_config_use_case: Arc<dyn ActivateWifiConfigUseCase>,
    pub delete_wifi_config_use_case: Arc<dyn DeleteWifiConfigUseCase>,
    pub create_static_ip_config_use_case: Arc<dyn CreateStaticIpConfigUseCase>,
    pub enable_static_ip_config_use_case: Arc<dyn EnableStaticIpConfigUseCase>,
    pub disable_static_ip_config_use_case: Arc<dyn DisableStaticIpConfigUseCase>,
    pub delete_static_ip_config_use_case: Arc<dyn DeleteStaticIpConfigUseCase>,
    pub scan_wifi_networks_use_case: Arc<dyn ScanWifiNetworksUseCase>,
}

// Create the router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(network_settings_handler))
        .route("/api/greetings", get(list_greetings_handler))
        .route("/api/greetings", post(create_greeting_handler))
        .route("/api/greetings/default", get(get_default_greeting_handler))
        // Network API handlers
        .route("/api/network/settings", get(get_network_settings_api_handler))
        .route("/api/network/wifi", post(create_wifi_config_handler))
        .route("/api/network/wifi/scan", get(scan_wifi_networks_handler))
        .route("/api/network/wifi/:id/activate", post(activate_wifi_config_handler))
        .route("/api/network/wifi/:id", delete(delete_wifi_config_handler))
        .route("/api/network/static-ip", post(create_static_ip_config_handler))
        .route("/api/network/static-ip/:id/enable", post(enable_static_ip_config_handler))
        .route("/api/network/static-ip/:id/disable", post(disable_static_ip_config_handler))
        .route("/api/network/static-ip/:id", delete(delete_static_ip_config_handler))
        .with_state(state)
}



// Network settings page handler
async fn network_settings_handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    match state.get_network_settings_use_case.execute().await {
        Ok(data) => {
            let wifi_configs_json = serde_json::to_string(&data.wifi_configs).unwrap_or_else(|_| "[]".to_string());
            let static_ip_configs_json = serde_json::to_string(&data.static_ip_configs).unwrap_or_else(|_| "[]".to_string());
            let interfaces_json = serde_json::to_string(&data.network_interfaces).unwrap_or_else(|_| "[]".to_string());
            let active_wifi_json = serde_json::to_string(&data.active_wifi).unwrap_or_else(|_| "null".to_string());
            
            let html = format!(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Homelabme - Network Settings</title>
                    <script src="https://cdn.tailwindcss.com"></script>
                    <script>
                        tailwind.config = {{
                            theme: {{
                                extend: {{
                                    colors: {{
                                        primary: '#667eea',
                                        secondary: '#764ba2'
                                    }}
                                }}
                            }}
                        }}
                    </script>
                </head>
                <body class="bg-gradient-to-br from-primary to-secondary min-h-screen">
                    <!-- Navigation -->
                    <nav class="bg-white/10 backdrop-blur-md border-b border-white/20">
                        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                            <div class="flex justify-between items-center h-16">
                                <div class="flex items-center space-x-4">
                                    <span class="text-2xl">🦀</span>
                                    <h1 class="text-xl font-bold text-white">Homelabme</h1>
                                </div>
                                <div class="flex space-x-4">
                                    <a href="/" class="text-white px-3 py-2 rounded-md text-sm font-medium bg-white/20 transition-colors">Settings</a>
                                </div>
                            </div>
                        </div>
                    </nav>

                    <!-- Main Content -->
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                        <div class="mb-8">
                            <h2 class="text-3xl font-bold text-white mb-2">Network Settings</h2>
                            <p class="text-white/80">Configure WiFi and static IP settings for your homelab server</p>
                        </div>

                        <!-- Network Interfaces Status -->
                        <div class="bg-white/10 backdrop-blur-md rounded-lg p-6 mb-8 border border-white/20">
                            <div class="flex items-center justify-between mb-4">
                                <h3 class="text-xl font-semibold text-white flex items-center">
                                    <span class="mr-2">🌐</span> Network Interfaces
                                </h3>
                                <div class="flex items-center space-x-2">
                                    <label for="interface-filter" class="text-sm text-white/90">Filter:</label>
                                    <select id="interface-filter" onchange="filterInterfaces()" 
                                            class="px-3 py-1 bg-white/20 border border-white/30 rounded-md text-white text-sm focus:outline-none focus:ring-2 focus:ring-white/50">
                                        <option value="up">UP Only</option>
                                        <option value="all">All Interfaces</option>
                                        <option value="down">DOWN Only</option>
                                    </select>
                                </div>
                            </div>
                            <div id="interfaces-list" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                                <!-- Interfaces will be populated by JavaScript -->
                            </div>
                        </div>

                        <!-- WiFi Configuration -->
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
                            <!-- WiFi Settings Form -->
                            <div class="bg-white/10 backdrop-blur-md rounded-lg p-6 border border-white/20">
                                <h3 class="text-xl font-semibold text-white mb-4 flex items-center">
                                    <span class="mr-2">📶</span> WiFi Configuration
                                </h3>
                                <form id="wifi-form" class="space-y-4">
                                    <div>
                                        <label for="wifi-ssid" class="block text-sm font-medium text-white/90 mb-2">Network Name (SSID)</label>
                                        <div class="flex space-x-2">
                                            <select id="wifi-ssid" name="ssid" required
                                                    class="flex-1 px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                                <option value="">Select a network...</option>
                                                <!-- Options will be populated by WiFi scan -->
                                            </select>
                                            <button type="button" onclick="scanWifiNetworks()" 
                                                    class="px-4 py-2 bg-blue-500/20 hover:bg-blue-500/30 text-white rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-blue-400/50">
                                                🔍 Scan
                                            </button>
                                        </div>
                                        <div class="mt-2">
                                            <input type="text" id="wifi-ssid-custom" placeholder="Or enter custom SSID..."
                                                   class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                        </div>
                                    </div>
                                    <div>
                                        <label for="wifi-password" class="block text-sm font-medium text-white/90 mb-2">Password</label>
                                        <input type="password" id="wifi-password" name="password" required
                                               class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                    </div>
                                    <div>
                                        <label for="wifi-security" class="block text-sm font-medium text-white/90 mb-2">Security Type</label>
                                        <select id="wifi-security" name="security_type"
                                                class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                            <option value="WPA2">WPA2</option>
                                            <option value="WPA3">WPA3</option>
                                            <option value="WPA">WPA</option>
                                            <option value="WEP">WEP</option>
                                            <option value="Open">Open</option>
                                        </select>
                                    </div>
                                    <button type="submit"
                                            class="w-full bg-white/20 hover:bg-white/30 text-white font-medium py-2 px-4 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-white/50">
                                        Add WiFi Configuration
                                    </button>
                                </form>
                            </div>

                            <!-- WiFi Configurations List -->
                            <div class="bg-white/10 backdrop-blur-md rounded-lg p-6 border border-white/20">
                                <h3 class="text-xl font-semibold text-white mb-4">Saved WiFi Networks</h3>
                                <div id="wifi-list" class="space-y-3">
                                    <!-- WiFi configs will be populated by JavaScript -->
                                </div>
                            </div>
                        </div>

                        <!-- Static IP Configuration -->
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                            <!-- Static IP Settings Form -->
                            <div class="bg-white/10 backdrop-blur-md rounded-lg p-6 border border-white/20">
                                <h3 class="text-xl font-semibold text-white mb-4 flex items-center">
                                    <span class="mr-2">🔧</span> Static IP Configuration
                                </h3>
                                <form id="static-ip-form" class="space-y-4">
                                    <div>
                                        <label for="interface-name" class="block text-sm font-medium text-white/90 mb-2">Network Interface</label>
                                        <select id="interface-name" name="interface_name" required
                                                class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                            <!-- Options will be populated by JavaScript -->
                                        </select>
                                    </div>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                        <div>
                                            <label for="ip-address" class="block text-sm font-medium text-white/90 mb-2">IP Address</label>
                                            <input type="text" id="ip-address" name="ip_address" required placeholder="192.168.1.100"
                                                   class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                        </div>
                                        <div>
                                            <label for="subnet-mask" class="block text-sm font-medium text-white/90 mb-2">Subnet Mask</label>
                                            <input type="text" id="subnet-mask" name="subnet_mask" required placeholder="255.255.255.0"
                                                   class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                        </div>
                                    </div>
                                    <div>
                                        <label for="gateway" class="block text-sm font-medium text-white/90 mb-2">Gateway</label>
                                        <input type="text" id="gateway" name="gateway" required placeholder="192.168.1.1"
                                               class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                    </div>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                        <div>
                                            <label for="dns-primary" class="block text-sm font-medium text-white/90 mb-2">Primary DNS</label>
                                            <input type="text" id="dns-primary" name="dns_primary" required placeholder="8.8.8.8"
                                                   class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                        </div>
                                        <div>
                                            <label for="dns-secondary" class="block text-sm font-medium text-white/90 mb-2">Secondary DNS</label>
                                            <input type="text" id="dns-secondary" name="dns_secondary" placeholder="8.8.4.4"
                                                   class="w-full px-3 py-2 bg-white/20 border border-white/30 rounded-md text-white placeholder-white/60 focus:outline-none focus:ring-2 focus:ring-white/50 focus:border-transparent">
                                        </div>
                                    </div>
                                    <button type="submit"
                                            class="w-full bg-white/20 hover:bg-white/30 text-white font-medium py-2 px-4 rounded-md transition-colors focus:outline-none focus:ring-2 focus:ring-white/50">
                                        Add Static IP Configuration
                                    </button>
                                </form>
                            </div>

                            <!-- Static IP Configurations List -->
                            <div class="bg-white/10 backdrop-blur-md rounded-lg p-6 border border-white/20">
                                <h3 class="text-xl font-semibold text-white mb-4">Static IP Configurations</h3>
                                <div id="static-ip-list" class="space-y-3">
                                    <!-- Static IP configs will be populated by JavaScript -->
                                </div>
                            </div>
                        </div>
                    </div>

                    <!-- Toast Notification -->
                    <div id="toast" class="fixed top-4 right-4 bg-green-500 text-white px-6 py-3 rounded-lg shadow-lg transform translate-x-full transition-transform duration-300 z-50">
                        <span id="toast-message"></span>
                    </div>

                    <script>
                        // Initial data from server
                        const wifiConfigs = {wifi_configs_json};
                        const staticIpConfigs = {static_ip_configs_json};
                        const networkInterfaces = {interfaces_json};
                        const activeWifi = {active_wifi_json};

                        // Toast notification function
                        function showToast(message, type = 'success') {{
                            const toast = document.getElementById('toast');
                            const toastMessage = document.getElementById('toast-message');
                            
                            toast.className = `fixed top-4 right-4 px-6 py-3 rounded-lg shadow-lg transform transition-transform duration-300 z-50 ${{type === 'success' ? 'bg-green-500' : 'bg-red-500'}} text-white`;
                            toastMessage.textContent = message;
                            toast.classList.remove('translate-x-full');
                            
                            setTimeout(() => {{
                                toast.classList.add('translate-x-full');
                            }}, 3000);
                        }}

                        // Store all interfaces globally for filtering
                        let allInterfaces = [...networkInterfaces];
                        let filteredInterfaces = [...networkInterfaces];

                        // Populate network interfaces
                        function populateInterfaces() {{
                            const interfacesList = document.getElementById('interfaces-list');
                            const interfaceSelect = document.getElementById('interface-name');
                            
                            interfacesList.innerHTML = '';
                            interfaceSelect.innerHTML = '';
                            
                            filteredInterfaces.forEach(iface => {{
                                // Build IP addresses display
                                let ipDisplay = '';
                                if (iface.ipv4_addresses && iface.ipv4_addresses.length > 0) {{
                                    ipDisplay += `<div><strong>IPv4:</strong> ${{iface.ipv4_addresses.join(', ')}}</div>`;
                                }}
                                if (iface.ipv6_addresses && iface.ipv6_addresses.length > 0) {{
                                    ipDisplay += `<div><strong>IPv6:</strong> ${{iface.ipv6_addresses.join(', ')}}</div>`;
                                }}
                                if (!ipDisplay && iface.current_ip) {{
                                    ipDisplay = `<div>IP: ${{iface.current_ip}}</div>`;
                                }}
                                
                                // Interface status card
                                const card = document.createElement('div');
                                card.className = 'bg-white/10 rounded-lg p-4 border border-white/20';
                                card.innerHTML = `
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="font-medium text-white">${{iface.name}}</span>
                                        <span class="px-2 py-1 rounded text-xs ${{iface.is_up ? 'bg-green-500/20 text-green-300' : 'bg-red-500/20 text-red-300'}}">
                                            ${{iface.is_up ? 'UP' : 'DOWN'}}
                                        </span>
                                    </div>
                                    <div class="text-sm text-white/70">
                                        <div>Type: ${{iface.interface_type}}</div>
                                        <div>MAC: ${{iface.mac_address}}</div>
                                        ${{ipDisplay}}
                                    </div>
                                `;
                                interfacesList.appendChild(card);
                            }});
                            
                            // Always populate select with all interfaces (not filtered)
                            allInterfaces.forEach(iface => {{
                                if (iface.interface_type !== 'Loopback') {{
                                    const option = document.createElement('option');
                                    option.value = iface.name;
                                    option.textContent = `${{iface.name}} (${{iface.interface_type}})`;
                                    interfaceSelect.appendChild(option);
                                }}
                            }});
                        }}

                        // Filter interfaces based on status
                        function filterInterfaces() {{
                            const filterValue = document.getElementById('interface-filter').value;
                            
                            switch(filterValue) {{
                                case 'up':
                                    filteredInterfaces = allInterfaces.filter(iface => iface.is_up);
                                    break;
                                case 'down':
                                    filteredInterfaces = allInterfaces.filter(iface => !iface.is_up);
                                    break;
                                case 'all':
                                default:
                                    filteredInterfaces = [...allInterfaces];
                                    break;
                            }}
                            
                            populateInterfaces();
                        }}

                        // Populate WiFi configurations
                        function populateWifiConfigs() {{
                            const wifiList = document.getElementById('wifi-list');
                            wifiList.innerHTML = '';
                            
                            if (wifiConfigs.length === 0) {{
                                wifiList.innerHTML = '<p class="text-white/60 text-sm">No WiFi configurations saved</p>';
                                return;
                            }}
                            
                            wifiConfigs.forEach(config => {{
                                const item = document.createElement('div');
                                item.className = `bg-white/10 rounded-lg p-4 border border-white/20 ${{config.is_active ? 'ring-2 ring-green-400' : ''}}`;
                                item.innerHTML = `
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="font-medium text-white">${{config.ssid}}</span>
                                        ${{config.is_active ? '<span class="px-2 py-1 bg-green-500/20 text-green-300 rounded text-xs">ACTIVE</span>' : ''}}
                                    </div>
                                    <div class="text-sm text-white/70 mb-3">
                                        Security: ${{config.security_type}}
                                    </div>
                                    <div class="flex space-x-2">
                                        ${{!config.is_active ? `<button onclick="activateWifi('${{config.id}}')" class="px-3 py-1 bg-blue-500/20 text-blue-300 rounded text-sm hover:bg-blue-500/30 transition-colors">Activate</button>` : ''}}
                                        <button onclick="deleteWifi('${{config.id}}')" class="px-3 py-1 bg-red-500/20 text-red-300 rounded text-sm hover:bg-red-500/30 transition-colors">Delete</button>
                                    </div>
                                `;
                                wifiList.appendChild(item);
                            }});
                        }}

                        // Populate Static IP configurations
                        function populateStaticIpConfigs() {{
                            const staticIpList = document.getElementById('static-ip-list');
                            staticIpList.innerHTML = '';
                            
                            if (staticIpConfigs.length === 0) {{
                                staticIpList.innerHTML = '<p class="text-white/60 text-sm">No static IP configurations saved</p>';
                                return;
                            }}
                            
                            staticIpConfigs.forEach(config => {{
                                const item = document.createElement('div');
                                item.className = `bg-white/10 rounded-lg p-4 border border-white/20 ${{config.is_enabled ? 'ring-2 ring-green-400' : ''}}`;
                                item.innerHTML = `
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="font-medium text-white">${{config.interface_name}}</span>
                                        <span class="px-2 py-1 rounded text-xs ${{config.is_enabled ? 'bg-green-500/20 text-green-300' : 'bg-gray-500/20 text-gray-300'}}">
                                            ${{config.is_enabled ? 'ENABLED' : 'DISABLED'}}
                                        </span>
                                    </div>
                                    <div class="text-sm text-white/70 mb-3">
                                        <div>IP: ${{config.ip_address}}/${{config.subnet_mask}}</div>
                                        <div>Gateway: ${{config.gateway}}</div>
                                        <div>DNS: ${{config.dns_primary}}${{config.dns_secondary ? `, ${{config.dns_secondary}}` : ''}}</div>
                                    </div>
                                    <div class="flex space-x-2">
                                        ${{config.is_enabled ? 
                                            `<button onclick="disableStaticIp('${{config.id}}')" class="px-3 py-1 bg-yellow-500/20 text-yellow-300 rounded text-sm hover:bg-yellow-500/30 transition-colors">Disable</button>` :
                                            `<button onclick="enableStaticIp('${{config.id}}')" class="px-3 py-1 bg-blue-500/20 text-blue-300 rounded text-sm hover:bg-blue-500/30 transition-colors">Enable</button>`
                                        }}
                                        <button onclick="deleteStaticIp('${{config.id}}')" class="px-3 py-1 bg-red-500/20 text-red-300 rounded text-sm hover:bg-red-500/30 transition-colors">Delete</button>
                                    </div>
                                `;
                                staticIpList.appendChild(item);
                            }});
                        }}



                        // Static IP form submission
                        document.getElementById('static-ip-form').addEventListener('submit', async (e) => {{
                            e.preventDefault();
                            const formData = new FormData(e.target);
                            const data = {{
                                interface_name: formData.get('interface_name'),
                                ip_address: formData.get('ip_address'),
                                subnet_mask: formData.get('subnet_mask'),
                                gateway: formData.get('gateway'),
                                dns_primary: formData.get('dns_primary'),
                                dns_secondary: formData.get('dns_secondary') || null
                            }};
                            
                            try {{
                                const response = await fetch('/api/network/static-ip', {{
                                    method: 'POST',
                                    headers: {{
                                        'Content-Type': 'application/json'
                                    }},
                                    body: JSON.stringify(data)
                                }});
                                
                                if (response.ok) {{
                                    showToast('Static IP configuration added successfully!');
                                    e.target.reset();
                                    setTimeout(() => location.reload(), 1000);
                                }} else {{
                                    showToast('Failed to add static IP configuration', 'error');
                                }}
                            }} catch (error) {{
                                showToast('Error adding static IP configuration', 'error');
                            }}
                        }});

                        // WiFi management functions
                        async function activateWifi(id) {{
                            try {{
                                const response = await fetch(`/api/network/wifi/${{id}}/activate`, {{
                                    method: 'POST'
                                }});
                                
                                if (response.ok) {{
                                    showToast('WiFi configuration activated!');
                                    setTimeout(() => location.reload(), 1000);
                                }} else {{
                                    showToast('Failed to activate WiFi configuration', 'error');
                                }}
                            }} catch (error) {{
                                showToast('Error activating WiFi configuration', 'error');
                            }}
                        }}

                        async function deleteWifi(id) {{
                            if (confirm('Are you sure you want to delete this WiFi configuration?')) {{
                                try {{
                                    const response = await fetch(`/api/network/wifi/${{id}}`, {{
                                        method: 'DELETE'
                                    }});
                                    
                                    if (response.ok) {{
                                        showToast('WiFi configuration deleted!');
                                        setTimeout(() => location.reload(), 1000);
                                    }} else {{
                                        showToast('Failed to delete WiFi configuration', 'error');
                                    }}
                                }} catch (error) {{
                                    showToast('Error deleting WiFi configuration', 'error');
                                }}
                            }}
                        }}

                        // Static IP management functions
                        async function enableStaticIp(id) {{
                            try {{
                                const response = await fetch(`/api/network/static-ip/${{id}}/enable`, {{
                                    method: 'POST'
                                }});
                                
                                if (response.ok) {{
                                    showToast('Static IP configuration enabled!');
                                    setTimeout(() => location.reload(), 1000);
                                }} else {{
                                    showToast('Failed to enable static IP configuration', 'error');
                                }}
                            }} catch (error) {{
                                showToast('Error enabling static IP configuration', 'error');
                            }}
                        }}

                        async function disableStaticIp(id) {{
                            try {{
                                const response = await fetch(`/api/network/static-ip/${{id}}/disable`, {{
                                    method: 'POST'
                                }});
                                
                                if (response.ok) {{
                                    showToast('Static IP configuration disabled!');
                                    setTimeout(() => location.reload(), 1000);
                                }} else {{
                                    showToast('Failed to disable static IP configuration', 'error');
                                }}
                            }} catch (error) {{
                                showToast('Error disabling static IP configuration', 'error');
                            }}
                        }}

                        async function deleteStaticIp(id) {{
                            if (confirm('Are you sure you want to delete this static IP configuration?')) {{
                                try {{
                                    const response = await fetch(`/api/network/static-ip/${{id}}`, {{
                                        method: 'DELETE'
                                    }});
                                    
                                    if (response.ok) {{
                                        showToast('Static IP configuration deleted!');
                                        setTimeout(() => location.reload(), 1000);
                                    }} else {{
                                        showToast('Failed to delete static IP configuration', 'error');
                                    }}
                                }} catch (error) {{
                                    showToast('Error deleting static IP configuration', 'error');
                                }}
                            }}
                        }}

                        // WiFi scanning functions
                        async function scanWifiNetworks() {{
                            const scanButton = document.querySelector('button[onclick="scanWifiNetworks()"]');
                            const originalText = scanButton.innerHTML;
                            
                            try {{
                                scanButton.innerHTML = '🔄 Scanning...';
                                scanButton.disabled = true;
                                
                                const response = await fetch('/api/network/wifi/scan');
                                
                                if (response.ok) {{
                                    const networks = await response.json();
                                    populateWifiNetworks(networks);
                                    showToast(`Found ${{networks.length}} WiFi networks`);
                                }} else {{
                                    showToast('Failed to scan WiFi networks', 'error');
                                }}
                            }} catch (error) {{
                                showToast('Error scanning WiFi networks', 'error');
                            }} finally {{
                                scanButton.innerHTML = originalText;
                                scanButton.disabled = false;
                            }}
                        }}

                        function populateWifiNetworks(networks) {{
                            const ssidSelect = document.getElementById('wifi-ssid');
                            
                            // Clear existing options except the first one
                            ssidSelect.innerHTML = '<option value="">Select a network...</option>';
                            
                            // Sort networks by signal strength (descending)
                            networks.sort((a, b) => b.signal_level - a.signal_level);
                            
                            networks.forEach(network => {{
                                const option = document.createElement('option');
                                option.value = network.ssid;
                                option.textContent = `${{network.ssid}} (${{network.security}}, ${{network.signal_level}}dBm)`;
                                ssidSelect.appendChild(option);
                            }});
                        }}

                        // Handle SSID selection (dropdown vs custom input)
                        function handleSsidSelection() {{
                            const ssidSelect = document.getElementById('wifi-ssid');
                            const customInput = document.getElementById('wifi-ssid-custom');
                            
                            if (ssidSelect.value) {{
                                customInput.value = '';
                                customInput.removeAttribute('required');
                                ssidSelect.setAttribute('required', 'required');
                            }} else {{
                                ssidSelect.removeAttribute('required');
                                customInput.setAttribute('required', 'required');
                            }}
                        }}

                        // Add event listeners for SSID selection
                        document.addEventListener('DOMContentLoaded', function() {{
                            const ssidSelect = document.getElementById('wifi-ssid');
                            const customInput = document.getElementById('wifi-ssid-custom');
                            
                            ssidSelect.addEventListener('change', handleSsidSelection);
                            customInput.addEventListener('input', function() {{
                                if (this.value) {{
                                    ssidSelect.value = '';
                                    ssidSelect.removeAttribute('required');
                                    this.setAttribute('required', 'required');
                                }} else {{
                                    this.removeAttribute('required');
                                    ssidSelect.setAttribute('required', 'required');
                                }}
                            }});
                            
                            // Modify WiFi form submission
                            document.getElementById('wifi-form').addEventListener('submit', async function(e) {{
                                e.preventDefault();
                                
                                const formData = new FormData(this);
                                const ssidSelect = document.getElementById('wifi-ssid');
                                const customInput = document.getElementById('wifi-ssid-custom');
                                
                                // Use custom SSID if provided, otherwise use selected SSID
                                const ssid = customInput.value || ssidSelect.value;
                                
                                if (!ssid) {{
                                    showToast('Please select a network or enter a custom SSID', 'error');
                                    return;
                                }}
                                
                                const wifiConfig = {{
                                    ssid: ssid,
                                    password: formData.get('password'),
                                    security_type: formData.get('security_type')
                                }};
                                
                                try {{
                                    const response = await fetch('/api/network/wifi', {{
                                        method: 'POST',
                                        headers: {{
                                            'Content-Type': 'application/json'
                                        }},
                                        body: JSON.stringify(wifiConfig)
                                    }});
                                    
                                    if (response.ok) {{
                                        showToast('WiFi configuration added successfully!');
                                        this.reset();
                                        ssidSelect.value = '';
                                        customInput.value = '';
                                        handleSsidSelection();
                                        setTimeout(() => location.reload(), 1000);
                                    }} else {{
                                        showToast('Failed to add WiFi configuration', 'error');
                                    }}
                                }} catch (error) {{
                                    showToast('Error adding WiFi configuration', 'error');
                                }}
                            }});
                        }});

                        // Initialize page with default filter (UP interfaces only)
                        filteredInterfaces = allInterfaces.filter(iface => iface.is_up);
                        populateInterfaces();
                        populateWifiConfigs();
                        populateStaticIpConfigs();
                    </script>
                </body>
                </html>
                "#,
                wifi_configs_json = wifi_configs_json,
                static_ip_configs_json = static_ip_configs_json,
                interfaces_json = interfaces_json,
                active_wifi_json = active_wifi_json
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

// Network API handlers
async fn get_network_settings_api_handler(State(state): State<AppState>) -> Result<Json<NetworkSettingsPageData>, StatusCode> {
    match state.get_network_settings_use_case.execute().await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_wifi_config_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateWifiConfigRequest>,
) -> Result<Json<WifiConfigResponse>, StatusCode> {
    match state.create_wifi_config_use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn activate_wifi_config_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.activate_wifi_config_use_case.execute(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_wifi_config_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.delete_wifi_config_use_case.execute(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_static_ip_config_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateStaticIpConfigRequest>,
) -> Result<Json<StaticIpConfigResponse>, StatusCode> {
    match state.create_static_ip_config_use_case.execute(request).await {
        Ok(response) => Ok(Json(response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn enable_static_ip_config_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.enable_static_ip_config_use_case.execute(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn disable_static_ip_config_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.disable_static_ip_config_use_case.execute(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_static_ip_config_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.delete_static_ip_config_use_case.execute(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn scan_wifi_networks_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<ScannedWifiNetworkDto>>, StatusCode> {
    match state.scan_wifi_networks_use_case.execute().await {
        Ok(networks) => Ok(Json(networks)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}