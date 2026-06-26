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
        let net = Networks::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();
        
        sys.refresh_all();
        SystemStatus {
            timestamp: 1782043200,
            cpu_status: fetch_cpu_data(&sys),
            
            ram_status: fetch_ram_data(&sys),

            disk_status: fetch_disk_data(&disks),

            battery_status: fetch_battery_data(&sys),
            
            network_status: fetch_network_data(&net)
        }
    }
}

fn kb_to_gb(kb: u64) -> f32 {
    (kb as f32)/ TO_GB
}

fn fetch_cpu_data(sys: &System) -> Cpu {
    Cpu {
        top_processes: vec![
            String::from("Acode"),
            String::from("Termux"),
            String::from("Rustc"),
                ],
        cpu_usage: 60.0,
        load_average: 1.2,
        cpu_temp: 39.8,
    }
}


fn fetch_ram_data(sys: &System) -> Ram {
    
    let ram_capacity = kb_to_gb(sys.total_memory());
    let ram_used = kb_to_gb(sys.used_memory());
    Ram {
        ram_capacity_gb: ram_capacity,
        ram_used_gb: ram_used,
    }
}


fn fetch_disk_data(disks: &Disks) -> Disk {
    let mut disk_capacity = 0;
    let mut disk_used = 0;
    for disk in disks.list() {
        disk_capacity = disk.total_space();
        disk_used = disk.available_space();
    }
    let disk_capacity = kb_to_gb(disk_capacity);
    let disk_used = kb_to_gb(disk_used);
    Disk {
        disk_capacity_gb: disk_capacity,
        disk_used_gb: disk_used,
        disk_io_mbps: 150.4,
    }
}


fn fetch_battery_data(sys: &System) -> Battery {
    Battery {
        status: BatteryStatus::Discharging,
        health_percentage: 92,
        battery_temp: 35.2,
    } 
} 

fn fetch_network_data(net: &Networks) -> Network {
    Network {
            download_speed_kbps: 310.0,
            upload_speed_kbps: 350.2,
            ping_ms: 25,
    } 
} 