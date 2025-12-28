// === Security Agent Daemon - Continuous Monitoring ===
// Run with: cargo run --bin agent-daemon
// Options: cargo run --bin agent-daemon -- --interval 300

use clap::Parser;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::time::SystemTime;

use security_agent::models::SystemInfo;
use security_agent::agent::{get_agent, Agent};

#[derive(Parser, Debug)]
#[command(name = "security-agent-daemon")]
#[command(about = "Continuous security monitoring agent daemon")]
struct Args {
    /// Collection interval in seconds (default: 300)
    #[arg(short, long, default_value = "300")]
    interval: u64,
    
    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

fn main() {
    let args = Args::parse();
    
    // Setup logging first (before any log calls)
    // Check for RUST_LOG env var first, then use CLI arg
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &args.log_level);
    }
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .init();
    
    // Get interval: check environment variable first, then use CLI arg
    let interval = if let Ok(env_interval) = std::env::var("AGENT_INTERVAL") {
        if let Ok(interval) = env_interval.parse::<u64>() {
            log::info!("Using interval from AGENT_INTERVAL environment variable: {} seconds", interval);
            interval
        } else {
            log::warn!("Invalid AGENT_INTERVAL value, using CLI/default: {} seconds", args.interval);
            args.interval
        }
    } else {
        args.interval
    };
    
    log::info!("Security Agent Daemon starting...");
    log::info!("Collection interval: {} seconds", interval);
    
    // Setup graceful shutdown handling
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        log::info!("Shutdown signal received, gracefully stopping...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");
    
    // Initialize agent
    let agent = get_agent();
    let interval_duration = Duration::from_secs(interval);
    let mut cycle_count = 0u64;
    
    log::info!("Starting continuous monitoring loop...");
    log::info!("Press Ctrl+C to stop\n");
    
    // Main monitoring loop
    while running.load(Ordering::SeqCst) {
        cycle_count += 1;
        let cycle_start = SystemTime::now();
        
        log::info!("=== Collection Cycle #{} ===", cycle_count);
        
        // Collect system information
        match collect_and_log(&*agent, cycle_count) {
            Ok(()) => {
                let elapsed = cycle_start.elapsed()
                    .unwrap_or(Duration::from_secs(0));
                log::info!("Cycle #{} completed in {:.2}s", cycle_count, elapsed.as_secs_f64());
            }
            Err(e) => {
                log::error!("Cycle #{} failed: {}", cycle_count, e);
            }
        }
        
        // Check if we should continue
        if !running.load(Ordering::SeqCst) {
            break;
        }
        
        // Wait for next cycle
        log::info!("Waiting {} seconds until next cycle...\n", interval);
        
        // Sleep in smaller chunks to allow for quicker shutdown response
        let sleep_chunk = Duration::from_secs(1);
        let mut remaining = interval_duration;
        
        while remaining.as_secs() > 0 && running.load(Ordering::SeqCst) {
            std::thread::sleep(sleep_chunk.min(remaining));
            if remaining > sleep_chunk {
                remaining -= sleep_chunk;
            } else {
                break;
            }
        }
    }
    
    log::info!("Daemon stopped. Total cycles completed: {}", cycle_count);
}

/// Collects system information and logs summary statistics
fn collect_and_log(agent: &dyn Agent, cycle: u64) -> anyhow::Result<()> {
    let system_info = agent.collect_system_info();
    
    // Log summary statistics
    log::info!(
        "Collection #{} summary: {} processes, {} connections, {} ports, {} users, {} services, {} tasks, {} packages",
        cycle,
        system_info.processes.len(),
        system_info.network_connections.len(),
        system_info.listening_ports.len(),
        system_info.users.len(),
        system_info.services.len(),
        system_info.scheduled_tasks.len(),
        system_info.installed_packages.len()
    );
    
    // Log OS and system info if available
    if let Some(os_version) = &system_info.os_version {
        log::debug!(
            "OS: {} {} ({})",
            os_version.name.as_deref().unwrap_or("Unknown"),
            os_version.version.as_deref().unwrap_or("Unknown"),
            os_version.arch.as_deref().unwrap_or("Unknown")
        );
    }
    
    if let Some(sys_info) = &system_info.system_info {
        if let Some(hostname) = &sys_info.hostname {
            log::debug!("Hostname: {}", hostname);
        }
    }
    
    // Log any warnings (empty collections might indicate issues)
    let mut warnings = Vec::new();
    
    if system_info.processes.is_empty() {
        warnings.push("No processes collected (OSquery may have issues)");
    }
    if system_info.network_connections.is_empty() && system_info.listening_ports.is_empty() {
        warnings.push("No network data collected");
    }
    if system_info.users.is_empty() {
        warnings.push("No users collected");
    }
    if system_info.services.is_empty() {
        warnings.push("No services collected");
    }
    
    for warning in warnings {
        log::warn!("{}", warning);
    }
    
    Ok(())
}

