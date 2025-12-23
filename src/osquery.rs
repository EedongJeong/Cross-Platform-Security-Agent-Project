// ============================================================================
// OSquery Integration Module
// ============================================================================

use std::process::Command;
use anyhow::{Result, Context};
use serde::Deserialize;
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

    // Basic debug output so we can see which binary and query are used
    eprintln!("[osquery] Executing '{}' using binary '{}'", query, osquery_path);

    let output = Command::new(&osquery_path)
        .arg("--json")
        .arg(query)
        .output()
        .with_context(|| format!("Failed to execute OSquery. Is OSquery installed? Tried: {}", osquery_path))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("[osquery] Query failed. stderr: {}", error_msg);
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
    let json_values = match execute_osquery_query(query) {
        Ok(values) => values,
        Err(e) => {
            eprintln!("[osquery] Error executing query '{}': {:?}", query, e);
            return Err(e);
        }
    };

    let mut results = Vec::new();
    
    for value in json_values {
        match serde_json::from_value::<T>(value.clone()) {
            Ok(parsed) => results.push(parsed),
            Err(e) => {
                eprintln!(
                    "[osquery] Failed to deserialize OSquery result for query '{}': {:?}\n  Value: {}",
                    query,
                    e,
                    value
                );
                // Skip this row but continue with others
            }
        }
    }
    
    Ok(results)
}

