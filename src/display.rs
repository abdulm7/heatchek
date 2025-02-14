use crate::utils::format_bytes;
use crate::system::SystemMonitor;

pub fn render_system_info(monitor: &SystemMonitor) {
    println!("╔══ System Monitor ══╗");

    // CPU Section
    println!("\n[CPU Status]");
    println!("Global CPU Usage: {:.1}%", monitor.global_cpu_usage());
    
    for (core_num, usage) in monitor.cpu_cores_usage() {
        println!("Core {}: {:.1}%", core_num, usage);
    }

    // Memory Section
    println!("\n[Memory Status]");
    let (total, used, free) = monitor.memory_info();
    println!("Total: {}", format_bytes(total));
    println!("Used:  {}", format_bytes(used));
    println!("Free:  {}", format_bytes(free));

    // System Info
    println!("\n[System Information]");
    let (name, kernel, host) = monitor.system_info();
    if let Some(name) = name {
        println!("OS: {}", name);
    }
    if let Some(kernel) = kernel {
        println!("Kernel: {}", kernel);
    }
    if let Some(host) = host {
        println!("Host: {}", host);
    }
}
