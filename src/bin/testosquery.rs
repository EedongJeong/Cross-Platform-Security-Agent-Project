// === OSquery Agent Test Tool ===
// Run with: cargo run --bin testosquery

// Use the library crate
use security_agent::models::SystemInfo;
use security_agent::agent::get_agent;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║         Security Agent - OSquery Test Tool                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    println!("Collecting system information via OSquery...\n");
    
    // Get platform-specific agent
    let agent = get_agent();
    
    // Collect system info
    println!("[INFO] Querying OSquery...");
    let system_info = agent.collect_system_info();
    
    // Display results
    println!("\n═══════════════════════════════════════════════════════════════");
    println!("                    COLLECTION RESULTS");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    // Print OS Version
    print_os_version(&system_info);
    
    // Print System Info
    print_system_info(&system_info);
    
    // Print summary counts
    print_summary(&system_info);
    
    // Print sample data
    print_sample_data(&system_info);
    
    // Print full JSON output option
    print_json_option(&system_info);
}

fn print_os_version(system_info: &SystemInfo) {
    println!("┌─ OS Version ─────────────────────────────────────────────────┐");
    if let Some(os_version) = &system_info.os_version {
        println!("│ Name:        {:50} │", 
                 os_version.name.as_deref().unwrap_or("N/A"));
        println!("│ Version:     {:50} │", 
                 os_version.version.as_deref().unwrap_or("N/A"));
        println!("│ Platform:    {:50} │", 
                 os_version.platform.as_deref().unwrap_or("N/A"));
        println!("│ Architecture: {:49} │", 
                 os_version.arch.as_deref().unwrap_or("N/A"));
        if let Some(build) = &os_version.build {
            println!("│ Build:       {:50} │", build);
        }
        if let Some(codename) = &os_version.codename {
            println!("│ Codename:    {:50} │", codename);
        }
    } else {
        println!("│ ⚠  OS Version: Not available                              │");
        println!("│    (OSquery may not be installed or accessible)           │");
    }
    println!("└──────────────────────────────────────────────────────────────┘\n");
}

fn print_system_info(system_info: &SystemInfo) {
    println!("┌─ System Information ─────────────────────────────────────────┐");
    if let Some(sys_info) = &system_info.system_info {
        if let Some(hostname) = &sys_info.hostname {
            println!("│ Hostname:    {:50} │", hostname);
        }
        if let Some(cpu_brand) = &sys_info.cpu_brand {
            println!("│ CPU Brand:   {:50} │", cpu_brand);
        }
        if let Some(mem) = sys_info.physical_memory {
            let mem_gb = mem as f64 / 1_073_741_824.0; // Convert to GB
            println!("│ Memory:      {:47.2} GB │", mem_gb);
        }
        if let (Some(phys), Some(log)) = (sys_info.cpu_physical_cores, sys_info.cpu_logical_cores) {
            println!("│ CPU Cores:   {:25} (Physical/Logical) │", 
                     format!("{}/{}", phys, log));
        }
        if let Some(uuid) = &sys_info.uuid {
            println!("│ System UUID: {:50} │", uuid);
        }
        if let Some(vendor) = &sys_info.hardware_vendor {
            println!("│ Vendor:      {:50} │", vendor);
        }
        if let Some(model) = &sys_info.hardware_model {
            println!("│ Model:       {:50} │", model);
        }
    } else {
        println!("│ ⚠  System Info: Not available                            │");
    }
    println!("└──────────────────────────────────────────────────────────────┘\n");
}

fn print_summary(system_info: &SystemInfo) {
    println!("┌─ Collection Summary ─────────────────────────────────────────┐");
    println!("│ Processes:           {:42} │", system_info.processes.len());
    println!("│ Network Connections: {:42} │", system_info.network_connections.len());
    println!("│ Listening Ports:     {:42} │", system_info.listening_ports.len());
    println!("│ Users:               {:42} │", system_info.users.len());
    println!("│ Services:            {:42} │", system_info.services.len());
    println!("│ Scheduled Tasks:     {:42} │", system_info.scheduled_tasks.len());
    println!("│ Installed Packages:  {:42} │", system_info.installed_packages.len());
    println!("│ Interface Addresses: {:42} │", system_info.interface_addresses.len());
    println!("└──────────────────────────────────────────────────────────────┘\n");
}

fn print_sample_data(system_info: &SystemInfo) {
    println!("┌─ Sample Data ─────────────────────────────────────────────────┐");
    
    // Show first 3 processes
    if !system_info.processes.is_empty() {
        println!("│ Top 3 Processes:                                            │");
        for (i, proc) in system_info.processes.iter().take(3).enumerate() {
            let name = proc.name.as_deref().unwrap_or("Unknown");
            let pid = proc.pid.map(|p| p.to_string()).unwrap_or_else(|| "N/A".to_string());
            println!("│   {}. {} (PID: {})", i + 1, 
                     truncate(name, 35), 
                     truncate(&pid, 10));
        }
        println!("│                                                              │");
    }
    
    // Show first 3 listening ports
    if !system_info.listening_ports.is_empty() {
        println!("│ Top 3 Listening Ports:                                     │");
        for (i, port) in system_info.listening_ports.iter().take(3).enumerate() {
            let port_num = port.port.map(|p| p.to_string()).unwrap_or_else(|| "N/A".to_string());
            let address = port.address.as_deref().unwrap_or("N/A");
            println!("│   {}. {}:{}", i + 1, address, port_num);
        }
        println!("│                                                              │");
    }
    
    // Show first 3 users
    if !system_info.users.is_empty() {
        println!("│ Sample Users:                                               │");
        for (i, user) in system_info.users.iter().take(3).enumerate() {
            let username = user.username.as_deref().unwrap_or("Unknown");
            let uid = user.uid.map(|u| u.to_string()).unwrap_or_else(|| "N/A".to_string());
            println!("│   {}. {} (UID: {})", i + 1, username, uid);
        }
    }
    
    println!("└──────────────────────────────────────────────────────────────┘\n");
}

fn print_json_option(system_info: &SystemInfo) {
    println!("═══════════════════════════════════════════════════════════════");
    println!("              FULL JSON OUTPUT (First 2000 chars)");
    println!("═══════════════════════════════════════════════════════════════\n");
    
    match serde_json::to_string_pretty(system_info) {
        Ok(json) => {
            let preview = if json.len() > 2000 {
                format!("{}\n\n... (truncated, full output is {} characters) ...", 
                       &json[..2000], json.len())
            } else {
                json
            };
            println!("{}", preview);
            println!("\n═══════════════════════════════════════════════════════════════");
            println!("Test completed successfully!");
            println!("═══════════════════════════════════════════════════════════════");
        },
        Err(e) => {
            println!("Error serializing to JSON: {}", e);
            println!("═══════════════════════════════════════════════════════════════");
        }
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

