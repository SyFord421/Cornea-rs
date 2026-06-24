use crate::model::SystemStatus;
use crate::model::{Battery, BatteryStatus, Cpu, Disk, Network, Ram};

// Dummy untuk sementara sampai aku menemukan dia 🥀

impl SystemStatus {
    pub fn read_system() -> Self {
        SystemStatus {
            timestamp: 1782043200,
            cpu_status: Cpu {
                top_processes: vec![
                    String::from("Acode"),
                    String::from("Termux"),
                    String::from("Rustc"),
                ],
                cpu_usage: 61.7,
                load_average: 1.2,
                cpu_temp: 39.8,
            },

            ram_status: Ram {
                ram_capacity_gb: 4.0,
                ram_used_gb: 2.3,
            },

            disk_status: Disk {
                disk_capacity_gb: 64,
                disk_used_gb: 39,
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
