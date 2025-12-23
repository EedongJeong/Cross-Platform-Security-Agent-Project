// === Cross-Platform Security Agent - Test Version ===

// Use the library crate
use security_agent::models::SystemInfo;
use security_agent::agent::get_agent;

fn main() {
    println!("=== Security Agent OSquery Test ===\n");
    println!("Collecting system information via OSquery...\n");
    
    // Get platform-specific agent
    let agent = get_agent();
    
    // Collect system info
    println!("Querying OSquery...");
    let system_info = agent.collect_system_info();
    
    // Display results
    println!("\n=== Collection Results ===\n");
    
    // Print OS Version
    if let Some(os_version) = &system_info.os_version {
        println!("OS Version:");
        println!("  Name: {:?}", os_version.name);
        println!("  Version: {:?}", os_version.version);
        println!("  Platform: {:?}", os_version.platform);
        println!("  Arch: {:?}", os_version.arch);
        println!();
    } else {
        println!("OS Version: Not available (OSquery may not be installed or accessible)\n");
    }
    
    // Print System Info
    if let Some(sys_info) = &system_info.system_info {
        println!("System Info:");
        println!("  Hostname: {:?}", sys_info.hostname);
        println!("  CPU Brand: {:?}", sys_info.cpu_brand);

        if let Some(mem_str) = &sys_info.physical_memory {
            if let Ok(bytes) = mem_str.parse::<f64>() {
                println!(
                    "  Physical Memory: {} bytes ({:.2} GB)",
                    mem_str,
                    bytes / 1_000_000_000.0
                );
            } else {
                println!("  Physical Memory: {} bytes (parse error)", mem_str);
            }
        } else {
            println!("  Physical Memory: Not available");
        }

        println!(
            "  CPU Cores (Physical/Logical): {:?}/{:?}",
            sys_info.cpu_physical_cores, sys_info.cpu_logical_cores
        );
        println!();
    } else {
        println!("System Info: Not available\n");
    }
    
    // Print summary counts
    println!("Collection Summary:");
    println!("  Processes: {}", system_info.processes.len());
    println!("  Network Connections: {}", system_info.network_connections.len());
    println!("  Listening Ports: {}", system_info.listening_ports.len());
    println!("  Users: {}", system_info.users.len());
    println!("  Services: {}", system_info.services.len());
    println!("  Scheduled Tasks: {}", system_info.scheduled_tasks.len());
    println!("  Installed Packages: {}", system_info.installed_packages.len());
    println!("  Interface Addresses: {}", system_info.interface_addresses.len());
    println!();
    
    // Print full JSON output
    println!("=== Full JSON Output ===\n");
    match serde_json::to_string_pretty(&system_info) {
        Ok(json) => println!("{}", json),
        Err(e) => println!("Error serializing to JSON: {}", e),
    }
}

