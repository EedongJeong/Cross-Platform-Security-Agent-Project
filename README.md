# Cross-Platform Security Agent

A Rust-based security agent that uses OSquery to collect comprehensive system information across Windows, Linux, and macOS platforms.

## Project Structure

```
src/
├── lib.rs              # Library crate (exports all modules)
├── main.rs             # Main application entry point
├── models.rs           # Data structures for OSquery table schemas
├── osquery.rs          # OSquery integration and query execution
├── agent.rs            # Agent trait and platform-specific implementations
└── bin/
    └── testosquery.rs  # Standalone test tool for OSquery (no Tauri)
```

## Modules

### `models.rs`
Contains all data structures that map to OSquery tables:
- `OsVersion` - Operating system version information
- `SystemDetails` - Hardware and system configuration
- `ProcessInfo` - Running process details
- `NetworkConnection` - Active network connections
- `ListeningPort` - Listening ports
- `UserInfo` - User account information
- `ServiceInfo` - System services/daemons
- `ScheduledTask` - Scheduled tasks/cron jobs
- `PackageInfo` - Installed packages
- `InterfaceAddress` - Network interface configurations
- `SystemInfo` - Comprehensive structure containing all collected data

### `osquery.rs`
OSquery integration module:
- `find_osquery_binary()` - Locates OSquery binary on the current platform
- `execute_osquery_query()` - Executes SQL queries via OSquery and returns JSON
- `query_to_struct()` - Deserializes OSquery JSON results into typed Rust structs

### `agent.rs`
Agent trait and platform-specific implementations:
- `Agent` trait - Common interface for all platforms
- `WindowsAgent` - Windows-specific implementation
- `LinuxAgent` - Linux-specific implementation
- `MacAgent` - macOS-specific implementation
- `get_agent()` - Factory function that returns the appropriate agent for the current platform

### `main.rs`
Entry point with a simple CLI test interface that:
- Collects system information using OSquery
- Displays summary statistics
- Outputs full JSON results

## Prerequisites

1. **Rust** - Install from [rustup.rs](https://rustup.rs/)
2. **OSquery** - Must be installed on the target system
   - Windows: Download from [osquery.io](https://osquery.io/downloads) or use Chocolatey: `choco install osquery`
   - Linux: Install via package manager or download from osquery.io
   - macOS: `brew install osquery` or download from osquery.io

## Building and Running

### Run the Test Tool (Recommended)

The project includes a dedicated test tool for testing OSquery integration without Tauri:

```bash
# Run the OSquery test tool
cargo run --bin testosquery

# Or build and run separately
cargo build --bin testosquery
./target/debug/testosquery  # Linux/macOS
# or
.\target\debug\testosquery.exe  # Windows
```

### Run the Main Application

```bash
# Build the project
cargo build

# Run the main application
cargo run

# Build release version
cargo build --release
```

### Project Structure

The project is organized as a library crate with multiple binaries:

- `src/lib.rs` - Library that exports all modules
- `src/main.rs` - Main application entry point
- `src/bin/testosquery.rs` - Standalone test tool for OSquery integration
- `src/models.rs` - Data structures
- `src/osquery.rs` - OSquery integration
- `src/agent.rs` - Agent implementations

## How It Works

1. The application detects the current operating system
2. Selects the appropriate agent implementation (Windows/Linux/macOS)
3. Executes multiple OSquery SQL queries to collect system data
4. Deserializes JSON results into typed Rust structures
5. Aggregates all data into a `SystemInfo` struct
6. Outputs the results (currently as console output and JSON)

## Error Handling

The implementation gracefully handles:
- Missing OSquery installation (returns empty results)
- Query execution failures (uses `unwrap_or_default()` for safe fallback)
- Missing or incomplete data (all fields are `Option<T>`)

## Platform-Specific Tables

Each platform queries different OSquery tables:

- **Windows**: `services`, `scheduled_tasks`, `programs`
- **Linux**: `systemd_units`, `crontab`, `rpm_packages`, `deb_packages`, etc.
- **macOS**: `launchd`, `crontab`, `homebrew_packages`, `macports_packages`

## Future Enhancements

- Add Tauri frontend integration
- Implement continuous monitoring
- Add data persistence/storage
- Implement alerting mechanisms
- Add filtering and query optimization

