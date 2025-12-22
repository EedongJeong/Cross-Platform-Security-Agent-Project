// ============================================================================
// Data Structures for OSquery Tables
// ============================================================================

use serde::{Deserialize, Serialize};

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

