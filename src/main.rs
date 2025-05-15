use std::{thread, time};
use sysinfo::{System, SystemExt, CpuExt, ComponentExt};
use chrono::Local; // Import Local for timestamp

fn main() {
    let mut system = System::new_all();

    loop {
        // Clear the console (platform-dependent)
        #[cfg(target_os = "windows")]
        std::process::Command::new("cls").status().unwrap_or_default();
        #[cfg(not(target_os = "windows"))]
        std::process::Command::new("clear").status().unwrap_or_default();

        system.refresh_all();

        println!("--- System Monitor ---");
        println!("Time: {}", Local::now().format("%Y-%m-%d %H:%M:%S").to_string());

        // Temperatures
        println!("\nTemperatures:");
        for component in system.components() {
            println!("  {}: {:.2}°C", component.label(), component.temperature());
        }

        // Memory
        println!("\nMemory:");
        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let free_memory = system.free_memory();
        println!("  Total: {:.2} GB", total_memory as f64 / 1024.0 / 1024.0 / 1024.0);
        println!("  Used: {:.2} GB ({:.2}%)",
                 used_memory as f64 / 1024.0 / 1024.0 / 1024.0,
                 (used_memory as f64 / total_memory as f64) * 100.0);
        println!("  Free: {:.2} GB", free_memory as f64 / 1024.0 / 1024.0 / 1024.0);

        // CPU Load
        println!("\nCPU Load:");
        for (i, cpu) in system.cpus().iter().enumerate() {
            println!("  CPU {}: {:.2}%", i + 1, cpu.cpu_usage()); // Closing parenthesis here
        }
        println!("  Total CPU Load: {:.2}%", system.global_cpu_info().cpu_usage());

        // Wait for 1 second
        thread::sleep(time::Duration::from_secs(1));
    }
}
