
// Made With ♥️ By SyFord 

// Static Data khusus data yang akan di panggil sekali di awal
#[derive(Debug)]
pub struct Device {
    pub name: String,
    pub timestamp: u32,
    
}

#[derive(Debug)]
pub struct Cpu {
    pub name: String,
    pub core: u8,
    
}

#[derive(Debug)]
pub struct Ram {
    pub ram_capacity_gb: f32,
}

#[derive(Debug)]
pub struct Disk {
    pub disk_capacity_gb: f32,
}

#[derive(Debug)]
pub struct Battery {
    pub health_percentage: u8,
}
// untuk bagian network tapi karena network butuh pemantauan terus menerus saya pindahan ke dynamic 

// Untuk Static Data utama...
#[derive(Debug)]
pub struct Staticdata {
    pub device: Device,
    pub cpu_status: Cpu,
    pub disk_status: Disk,
    pub ram_status: Ram,
    pub battery_status: Battery,
}




// Dynamic data Khusus untuk data yang di panggil terus menerus 

#[derive(Debug)]
pub enum BatteryStatus {
    Charging,
    Discharging,
}

#[derive(Debug)]
pub struct DynCpu {
    pub top_processes: Vec<String>,
    pub cpu_usage: f32,
    pub load_average: f32,
    pub cpu_temp: f32,
    
}

#[derive(Debug)]
pub struct DynRam {
    pub ram_used_gb: f32, 
}


#[derive(Debug)]
pub struct DynDisk {
    pub disk_used_gb: f32,
    pub disk_io_mbps: f32,
}

#[derive(Debug)]
pub struct DynBattery {
    pub status: BatteryStatus, // Misal: "Charging", "Discharging"
    pub battery_temp: f32, 
}

#[derive(Debug)]
pub struct DynNetwork {
    pub download_speed_kbps: f32,
    pub upload_speed_kbps: f32,
    pub ping_ms: u32,
}

// untuk data dynamic utama..
#[derive(Debug)]    
pub struct Dynamicdata {
    pub cpu_status: DynCpu,
    pub ram_status: DynRam,
    pub disk_status: DynDisk,   
    pub battery_status: DynBattery,
    pub network_status: DynNetwork,
}