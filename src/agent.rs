// ============================================================================
// Agent Trait and Platform-Specific Implementations
// ============================================================================

use crate::models::*;
use crate::osquery::query_to_struct;

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

