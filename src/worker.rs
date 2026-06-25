use crate::model::SystemStatus;
use crate::model::{Battery, BatteryStatus, Cpu, Disk, Network, Ram};
use sysinfo::{
    Components, Disks, Networks, System,
};


const TO_GB:f32 = 1024.0 * 1024.0 * 1024.0;
// //Walaupun sudah bukan dummy tetap saja aku belum menemukan dia 😔

impl SystemStatus {
    pub fn read_system() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        //CPU
        let cpu_usage = sys.global_cpu_usage();// menghitung rata-rata dari semua core cpu
        
        //RAM
        let ram_capacity = kb_to_gb(sys.total_memory());
        let ram_used = kb_to_gb(sys.used_memory());

        //DISK
        let mut disk_capacity: f32 = 0.0;
        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            disk_capacity = kb_to_gb(disk.total_space());
        }
        
        
        SystemStatus {
            timestamp: 1782043200,
            cpu_status: Cpu {
                top_processes: vec![
                    String::from("Acode"),
                    String::from("Termux"),
                    String::from("Rustc"),
                ],
                cpu_usage: cpu_usage,
                load_average: 1.2,
                cpu_temp: 39.8,
            },

            ram_status: Ram {
                ram_capacity_gb: ram_capacity,
                ram_used_gb: ram_used,
            },

            disk_status: Disk {
                disk_capacity_gb: disk_capacity,
                disk_used_gb: 39.0,
                disk_io_mbps: 150.4,
            },

            battery_status: Battery {
                status: BatteryStatus::Discharging,
                health_percentage: 92,
                battery_temp: 35.2,
            },
            network_status: Network {
                download_speed_kbps: 310.0,
                upload_speed_kbps: 350.2,
                ping_ms: 25,
            },
        }
    }
}

fn kb_to_gb(kb: u64) -> f32 {
    (kb as f32)/ TO_GB
}