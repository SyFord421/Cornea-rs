#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Discharging,
}

#[derive(Debug)]
pub struct Cpu {
    pub top_processes: Vec<String>, // untuk daftar proses yang berjalan
    pub cpu_usage: f32,
    pub load_average: f32,
    pub cpu_temp: f32,
}
#[derive(Debug)]
pub struct Ram {
    pub ram_capacity_gb: f32,
    pub ram_used_gb: f32,
}

#[derive(Debug)]
pub struct Disk {
    pub disk_capacity_gb: f32,
    pub disk_used_gb: f32,
    pub disk_io_mbps: f32,
}

#[derive(Debug)]
pub struct Battery {
    pub status: BatteryStatus, // Misal: "Charging", "Discharging"
    pub health_percentage: u8,
    pub battery_temp: f32,
}

#[derive(Debug)]
pub struct Network {
    pub download_speed_kbps: f32,
    pub upload_speed_kbps: f32,
    pub ping_ms: u32,
}

#[derive(Debug)]
pub struct SystemStatus {
    pub timestamp: u64,
    pub cpu_status: Cpu,
    pub disk_status: Disk,
    pub ram_status: Ram,
    pub battery_status: Battery,
    pub network_status: Network,
}
