use sysinfo::{System, SystemExt};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("System Information:");
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory: {} KB", sys.used_memory());
    println!("Total swap: {} KB", sys.total_swap());
    println!("Used swap: {} KB", sys.used_swap());
    println!("System name: {:?}", sys.name());
    println!("System kernel version: {:?}", sys.kernel_version());
    println!("System OS version: {:?}", sys.os_version());
    println!("System host name: {:?}", sys.host_name());
}
