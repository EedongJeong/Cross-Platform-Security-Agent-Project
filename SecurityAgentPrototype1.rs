// === Cross-Platform Security Agent Structure using Rust and Tauri ===

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::process::Command;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// ============================================================================
// Data Structures for OSquery Tables
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsVersion {
    pub name: Option<String>,
    pub version: Option<String>,
    pub major: Option<i64>,
    pub minor: Option<i64>,
    pub patch: Option<i64>,
    pub build: Option<String>,
    pub platform: Option<String>,
    #[serde(rename = "platform_like")]
    pub platform_like: Option<String>,
    pub codename: Option<String>,
    pub arch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemDetails {
    pub hostname: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "cpu_type")]
    pub cpu_type: Option<String>,
    #[serde(rename = "cpu_subtype")]
    pub cpu_subtype: Option<String>,
    #[serde(rename = "cpu_brand")]
    pub cpu_brand: Option<String>,
    #[serde(rename = "cpu_physical_cores")]
    pub cpu_physical_cores: Option<i64>,
    #[serde(rename = "cpu_logical_cores")]
    pub cpu_logical_cores: Option<i64>,
    #[serde(rename = "cpu_microcode")]
    pub cpu_microcode: Option<String>,
    #[serde(rename = "physical_memory")]
    pub physical_memory: Option<i64>,
    #[serde(rename = "hardware_vendor")]
    pub hardware_vendor: Option<String>,
    #[serde(rename = "hardware_model")]
    pub hardware_model: Option<String>,
    #[serde(rename = "hardware_version")]
    pub hardware_version: Option<String>,
    #[serde(rename = "hardware_serial")]
    pub hardware_serial: Option<String>,
    #[serde(rename = "computer_name")]
    pub computer_name: Option<String>,
    #[serde(rename = "local_hostname")]
    pub local_hostname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessInfo {
    pub pid: Option<i64>,
    pub name: Option<String>,
    pub path: Option<String>,
    pub cmdline: Option<String>,
    pub state: Option<String>,
    pub cwd: Option<String>,
    pub root: Option<String>,
    pub uid: Option<i64>,
    pub gid: Option<i64>,
    pub euid: Option<i64>,
    pub egid: Option<i64>,
    pub suid: Option<i64>,
    pub sgid: Option<i64>,
    #[serde(rename = "on_disk")]
    pub on_disk: Option<i64>,
    #[serde(rename = "wired_size")]
    pub wired_size: Option<i64>,
    #[serde(rename = "resident_size")]
    pub resident_size: Option<i64>,
    #[serde(rename = "total_size")]
    pub total_size: Option<i64>,
    #[serde(rename = "user_time")]
    pub user_time: Option<i64>,
    #[serde(rename = "system_time")]
    pub system_time: Option<i64>,
    #[serde(rename = "disk_bytes_read")]
    pub disk_bytes_read: Option<i64>,
    #[serde(rename = "disk_bytes_written")]
    pub disk_bytes_written: Option<i64>,
    #[serde(rename = "start_time")]
    pub start_time: Option<i64>,
    pub parent: Option<i64>,
    pub pgroup: Option<i64>,
    pub threads: Option<i64>,
    pub nice: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkConnection {
    pub pid: Option<i64>,
    pub fd: Option<i64>,
    pub socket: Option<i64>,
    pub family: Option<i64>,
    pub protocol: Option<i64>,
    #[serde(rename = "local_address")]
    pub local_address: Option<String>,
    #[serde(rename = "local_port")]
    pub local_port: Option<i64>,
    #[serde(rename = "remote_address")]
    pub remote_address: Option<String>,
    #[serde(rename = "remote_port")]
    pub remote_port: Option<i64>,
    pub state: Option<String>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListeningPort {
    pub pid: Option<i64>,
    pub port: Option<i64>,
    pub protocol: Option<i64>,
    pub family: Option<i64>,
    pub address: Option<String>,
    pub fd: Option<i64>,
    pub socket: Option<i64>,
    pub path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub uid: Option<i64>,
    pub gid: Option<i64>,
    #[serde(rename = "uid_signed")]
    pub uid_signed: Option<i64>,
    #[serde(rename = "gid_signed")]
    pub gid_signed: Option<i64>,
    pub username: Option<String>,
    pub description: Option<String>,
    pub directory: Option<String>,
    pub shell: Option<String>,
    pub uuid: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "is_hidden")]
    pub is_hidden: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceInfo {
    pub name: Option<String>,
    #[serde(rename = "service_type")]
    pub service_type: Option<String>,
    #[serde(rename = "display_name")]
    pub display_name: Option<String>,
    pub status: Option<String>,
    pub pid: Option<i64>,
    #[serde(rename = "start_type")]
    pub start_type: Option<String>,
    #[serde(rename = "win32_exit_code")]
    pub win32_exit_code: Option<i64>,
    #[serde(rename = "service_exit_code")]
    pub service_exit_code: Option<i64>,
    pub path: Option<String>,
    #[serde(rename = "module_path")]
    pub module_path: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "user_account")]
    pub user_account: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScheduledTask {
    pub name: Option<String>,
    pub action: Option<String>,
    pub path: Option<String>,
    pub enabled: Option<i64>,
    pub state: Option<String>,
    pub hidden: Option<i64>,
    #[serde(rename = "last_run_time")]
    pub last_run_time: Option<i64>,
    #[serde(rename = "next_run_time")]
    pub next_run_time: Option<i64>,
    #[serde(rename = "last_run_message")]
    pub last_run_message: Option<String>,
    #[serde(rename = "last_run_code")]
    pub last_run_code: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub release: Option<String>,
    pub source: Option<String>,
    pub size: Option<i64>,
    pub sha1: Option<String>,
    pub arch: Option<String>,
    pub revision: Option<String>,
    pub status: Option<String>,
    pub maintainer: Option<String>,
    pub section: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InterfaceAddress {
    pub interface: Option<String>,
    pub address: Option<String>,
    pub mask: Option<String>,
    pub broadcast: Option<String>,
    #[serde(rename = "point_to_point")]
    pub point_to_point: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "friendly_name")]
    pub friendly_name: Option<String>,
}

/// Comprehensive system information structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub os_version: Option<OsVersion>,
    pub system_info: Option<SystemDetails>,
    pub processes: Vec<ProcessInfo>,
    pub network_connections: Vec<NetworkConnection>,
    pub listening_ports: Vec<ListeningPort>,
    pub users: Vec<UserInfo>,
    pub services: Vec<ServiceInfo>,
    pub scheduled_tasks: Vec<ScheduledTask>,
    pub installed_packages: Vec<PackageInfo>,
    pub interface_addresses: Vec<InterfaceAddress>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        SystemInfo {
            os_version: None,
            system_info: None,
            processes: Vec::new(),
            network_connections: Vec::new(),
            listening_ports: Vec::new(),
            users: Vec::new(),
            services: Vec::new(),
            scheduled_tasks: Vec::new(),
            installed_packages: Vec::new(),
            interface_addresses: Vec::new(),
        }
    }
}

// ============================================================================
// OSquery Integration Module
// ============================================================================

/// OSquery query executor module
mod osquery_executor {
    use super::*;
    use serde_json::Value;

    /// Finds the OSquery binary path based on platform
    fn find_osquery_binary() -> String {
        #[cfg(target_os = "windows")]
        {
            // Try common Windows paths
            let common_paths = vec![
                r"C:\Program Files\osquery\osqueryi.exe",
                r"C:\Program Files (x86)\osquery\osqueryi.exe",
            ];
            
            for path in common_paths {
                if std::path::Path::new(path).exists() {
                    return path.to_string();
                }
            }
            // Fallback to PATH lookup
            "osqueryi.exe".to_string()
        }
        
        #[cfg(target_os = "linux")]
        {
            let common_paths = vec![
                "/usr/bin/osqueryi",
                "/usr/local/bin/osqueryi",
                "/opt/osquery/bin/osqueryi",
            ];
            
            for path in common_paths {
                if std::path::Path::new(path).exists() {
                    return path.to_string();
                }
            }
            "osqueryi".to_string()
        }
        
        #[cfg(target_os = "macos")]
        {
            let common_paths = vec![
                "/usr/local/bin/osqueryi",
                "/opt/osquery/bin/osqueryi",
                "/usr/bin/osqueryi",
            ];
            
            for path in common_paths {
                if std::path::Path::new(path).exists() {
                    return path.to_string();
                }
            }
            "osqueryi".to_string()
        }
        
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        {
            "osqueryi".to_string()
        }
    }

    /// Executes an OSquery query and returns JSON result
    pub fn execute_osquery_query(query: &str) -> Result<Vec<Value>> {
        let osquery_path = find_osquery_binary();
        
        let output = Command::new(&osquery_path)
            .arg("--json")
            .arg(query)
            .output()
            .with_context(|| format!("Failed to execute OSquery. Is OSquery installed? Tried: {}", osquery_path))?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("OSquery query failed: {}", error_msg));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: Vec<Value> = serde_json::from_str(&stdout)
            .with_context(|| format!("Failed to parse OSquery JSON output: {}", stdout))?;

        Ok(json)
    }

    /// Executes a query and attempts to deserialize to a specific type
    pub fn query_to_struct<T>(query: &str) -> Result<Vec<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let json_values = execute_osquery_query(query)?;
        let mut results = Vec::new();
        
        for value in json_values {
            let parsed: T = serde_json::from_value(value)
                .context("Failed to deserialize OSquery result")?;
            results.push(parsed);
        }
        
        Ok(results)
    }
}

// Re-export for convenience
use osquery_executor::{execute_osquery_query, query_to_struct};

// ============================================================================
// Agent Trait
// ============================================================================

/// Trait common to all supported operating systems
pub trait Agent {
    /// Gather system info using OSquery
    fn collect_system_info(&self) -> SystemInfo;
}

/// Windows implementation
#[cfg(target_os = "windows")]
pub struct WindowsAgent {}

#[cfg(target_os = "windows")]
impl Agent for WindowsAgent {
    fn collect_system_info(&self) -> SystemInfo {
        SystemInfo {
            os_version: query_to_struct::<OsVersion>("SELECT * FROM os_version;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            system_info: query_to_struct::<SystemDetails>("SELECT * FROM system_info;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            processes: query_to_struct::<ProcessInfo>("SELECT * FROM processes;")
                .unwrap_or_default(),
            
            network_connections: query_to_struct::<NetworkConnection>(
                "SELECT * FROM process_open_sockets;"
            ).unwrap_or_default(),
            
            listening_ports: query_to_struct::<ListeningPort>(
                "SELECT * FROM listening_ports;"
            ).unwrap_or_default(),
            
            users: query_to_struct::<UserInfo>(
                "SELECT * FROM users;"
            ).unwrap_or_default(),
            
            services: query_to_struct::<ServiceInfo>(
                "SELECT * FROM services;"
            ).unwrap_or_default(),
            
            scheduled_tasks: query_to_struct::<ScheduledTask>(
                "SELECT * FROM scheduled_tasks;"
            ).unwrap_or_default(),
            
            installed_packages: query_to_struct::<PackageInfo>(
                "SELECT * FROM programs;"
            ).unwrap_or_default(),
            
            interface_addresses: query_to_struct::<InterfaceAddress>(
                "SELECT * FROM interface_addresses;"
            ).unwrap_or_default(),
        }
    }
}

/// Linux implementation
#[cfg(target_os = "linux")]
pub struct LinuxAgent {}

#[cfg(target_os = "linux")]
impl Agent for LinuxAgent {
    fn collect_system_info(&self) -> SystemInfo {
        SystemInfo {
            os_version: query_to_struct::<OsVersion>("SELECT * FROM os_version;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            system_info: query_to_struct::<SystemDetails>("SELECT * FROM system_info;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            processes: query_to_struct::<ProcessInfo>("SELECT * FROM processes;")
                .unwrap_or_default(),
            
            network_connections: query_to_struct::<NetworkConnection>(
                "SELECT * FROM process_open_sockets;"
            ).unwrap_or_default(),
            
            listening_ports: query_to_struct::<ListeningPort>(
                "SELECT * FROM listening_ports;"
            ).unwrap_or_default(),
            
            users: query_to_struct::<UserInfo>(
                "SELECT * FROM users;"
            ).unwrap_or_default(),
            
            services: query_to_struct::<ServiceInfo>(
                "SELECT * FROM systemd_units;"
            ).unwrap_or_default(),
            
            scheduled_tasks: query_to_struct::<ScheduledTask>(
                "SELECT * FROM crontab;"
            ).unwrap_or_default(),
            
            installed_packages: {
                // Try different package managers, collect all results
                let mut packages = Vec::new();
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM rpm_packages;").unwrap_or_default());
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM deb_packages;").unwrap_or_default());
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM portage_packages;").unwrap_or_default());
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM pkg_packages;").unwrap_or_default());
                packages
            },
            
            interface_addresses: query_to_struct::<InterfaceAddress>(
                "SELECT * FROM interface_addresses;"
            ).unwrap_or_default(),
        }
    }
}

/// MacOS implementation
#[cfg(target_os = "macos")]
pub struct MacAgent {}

#[cfg(target_os = "macos")]
impl Agent for MacAgent {
    fn collect_system_info(&self) -> SystemInfo {
        SystemInfo {
            os_version: query_to_struct::<OsVersion>("SELECT * FROM os_version;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            system_info: query_to_struct::<SystemDetails>("SELECT * FROM system_info;")
                .ok()
                .and_then(|v| v.into_iter().next()),
            
            processes: query_to_struct::<ProcessInfo>("SELECT * FROM processes;")
                .unwrap_or_default(),
            
            network_connections: query_to_struct::<NetworkConnection>(
                "SELECT * FROM process_open_sockets;"
            ).unwrap_or_default(),
            
            listening_ports: query_to_struct::<ListeningPort>(
                "SELECT * FROM listening_ports;"
            ).unwrap_or_default(),
            
            users: query_to_struct::<UserInfo>(
                "SELECT * FROM users;"
            ).unwrap_or_default(),
            
            services: query_to_struct::<ServiceInfo>(
                "SELECT * FROM launchd;"
            ).unwrap_or_default(),
            
            scheduled_tasks: query_to_struct::<ScheduledTask>(
                "SELECT * FROM crontab;"
            ).unwrap_or_default(),
            
            installed_packages: {
                // Try different package managers, collect all results
                let mut packages = Vec::new();
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM homebrew_packages;").unwrap_or_default());
                packages.extend(query_to_struct::<PackageInfo>("SELECT * FROM macports_packages;").unwrap_or_default());
                packages
            },
            
            interface_addresses: query_to_struct::<InterfaceAddress>(
                "SELECT * FROM interface_addresses;"
            ).unwrap_or_default(),
        }
    }
}

/// Returns the correct agent for the platform
pub fn get_agent() -> Box<dyn Agent> {
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsAgent {})
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxAgent {})
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(MacAgent {})
    }
}

// Tauri command for invoking system info collection
#[tauri::command]
fn request_system_info() -> String {
    let agent = get_agent();
    let system_info = agent.collect_system_info();
    
    serde_json::to_string(&system_info)
        .unwrap_or_else(|e| format!("{{\"error\": \"Failed to serialize system info: {}\"}}", e))
}

fn main() {
    // Setup Tauri Menu (optional, can be extended)
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submenu = Submenu::new("File", Menu::new().add_item(quit));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(submenu);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![request_system_info])
        .menu(menu)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



