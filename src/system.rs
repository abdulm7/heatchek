use sysinfo::{CpuExt, System, SystemExt};

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self { sys }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn global_cpu_usage(&self) -> f32 {
        self.sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / self.sys.cpus().len() as f32
    }

    pub fn cpu_cores_usage(&self) -> Vec<(usize, f32)> {
        self.sys.cpus()
            .iter()
            .enumerate()
            .map(|(i, cpu)| (i + 1, cpu.cpu_usage()))
            .collect()
    }

    pub fn memory_info(&self) -> (u64, u64, u64) {
        (
            self.sys.total_memory(),
            self.sys.used_memory(),
            self.sys.total_memory() - self.sys.used_memory(),
        )
    }

    pub fn system_info(&self) -> (Option<String>, Option<String>, Option<String>) {
        (self.sys.name(), self.sys.kernel_version(), self.sys.host_name())
    }
}
