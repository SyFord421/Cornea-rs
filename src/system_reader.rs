use crate::data_structure::SystemStatus;
use crate::data_structure::{Cpu, Ram, Disk, Battery, BatteryStatus, Network};

// Dummy untuk sementara sampai aku menemukan dia 🥀
pub fn read_system() -> SystemStatus {
         SystemStatus {
            timestamp: 1782043200,
            cpu_status: Cpu {
                top_processes: vec![String::from("termux"), String::from("rustc")],
                cpu_usage: 45.5,
                load_average: 1.2,
                cpu_temp: 39.8,
            },
        
            ram_status: Ram {
                ram_capacity_gb: 8,
                ram_used_gb: 4,         
            },
        
            disk_status: Disk {
                disk_capacity_gb: 128,
                disk_used_gb: 85,
                disk_io_mbps: 250.4,       
            },
        
            battery_status: Battery {
                status: BatteryStatus::Discharging,
                health_percentage: 92,
                battery_temp: 35.2,
            },
            network_status: Network {
                download_speed_kbps: 1500.5,
                upload_speed_kbps: 350.2,
                ping_ms: 24,
            },
        }
    }


