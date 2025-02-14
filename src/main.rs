use std::{thread, time::{Duration, Instant}};
use heatchek::{system::SystemMonitor, display, utils};

fn main() {
    let mut monitor = SystemMonitor::new();
    let start_time = Instant::now();
    
    while start_time.elapsed().as_secs() < 10 {
        utils::clear_screen();
        monitor.refresh();
        
        println!("Time remaining: {}s", 10 - start_time.elapsed().as_secs());
        display::render_system_info(&monitor);
        
        thread::sleep(Duration::from_secs(1));
    }
}
