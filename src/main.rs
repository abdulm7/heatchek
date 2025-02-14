use std::{
    thread,
    time::{Duration, Instant},
};
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    let start_time = Instant::now();

    while start_time.elapsed().as_secs() < 10 {
        // Clear screen (works on Unix-like systems and Windows)
        print!("\x1B[2J\x1B[1;1H");

        sys.refresh_all();

        println!("System Information (updating for 10 seconds):");
        println!("Time elapsed: {}s", start_time.elapsed().as_secs());

        // Add CPU information
        println!("\nCPU Information:");
        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("CPU {}: {:.2}%", i + 1, cpu.cpu_usage());
        }
        println!("\nMemory Information:");

        println!("Total memory: {} KB", sys.total_memory());
        println!("Used memory: {} KB", sys.used_memory());
        println!("System name: {:?}", sys.name());
        println!("System kernel version: {:?}", sys.kernel_version());
        println!("System OS version: {:?}", sys.os_version());
        println!("System host name: {:?}", sys.host_name());

        // Sleep for 1 second before next update
        thread::sleep(Duration::from_secs(1));
    }
}
