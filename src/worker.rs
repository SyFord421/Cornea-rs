use crate::model::{
    Staticdata, Dynamicdata, BatteryStatus, 
    Device, Battery, Cpu, Disk, Ram,
    DynCpu, DynRam, DynDisk, DynBattery, DynNetwork
};

use sysinfo::{
    Components, Disks, Networks, System,
};

// Rumus konversi KB ke GB yang bener (karena sysinfo balikinnya Bytes/KB tergantung fungsionalitasnya)
// Di sini kita asumsikan inputnya Bytes, jadi dibagi 1024^3
const TO_GB:f32 = 1024.0 * 1024.0 * 1024.0;
//Walaupun sudah bukan dummy tetap saja aku belum menemukan dia 😔

impl Staticdata {
    pub fn read_system(sys: &mut System, disk: &Disks) -> Self {      
        sys.refresh_all();
        Staticdata {
            device : fetch_device_data(&sys),
            
            cpu_status: fetch_cpu_data(&sys),
            
            ram_status: fetch_ram_data(&sys),

            disk_status: fetch_disk_data(&disk),
            
            battery_status: fetch_battery_data(),
        }
    }
}

fn kb_to_gb(kb: u64) -> f32 {
    (kb as f32)/ TO_GB
}

fn fetch_device_data(sys: &System) -> Device {
    Device {
        name: String::from("Redmi 9C"),
        timestamp: 1782043200,
    }
}

//✓
fn fetch_cpu_data(sys: &System) -> Cpu {
    Cpu {
        name: String::from("Helio G35"),
        core: 4,
    }
}


fn fetch_ram_data(sys: &System) -> Ram {
    Ram {
        ram_capacity_gb: 8.0,
    }
}


fn fetch_disk_data(disk: &Disks) -> Disk {
    Disk {
        disk_capacity_gb: 64.0,
    }
}

fn fetch_battery_data() -> Battery {
    Battery {
    health_percentage: 90,
    }
}


// Tadinya buat batrai Tapi udah di pindahin ke lain hati

// [||10%  ]=•


// (⁠╥⁠﹏⁠╥⁠) Kuper(Kurang perhatian)

impl Dynamicdata {
    pub fn read_system(sys: &System, net: &Networks, disk: &Disks) -> Self {
        
        Dynamicdata {
            cpu_status: fetch_dyncpu_data(),
            ram_status: fetch_dynram_data(),
            disk_status: fetch_dyndisk_data(),
            battery_status: fetch_dynbattery_data(),
            network_status: fetch_dynnetwork_data(),
        }
    }
}



fn fetch_dyncpu_data() -> DynCpu {
    DynCpu {
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

fn fetch_dynram_data() -> DynRam {
    DynRam {
        ram_used_gb: 2.6,
    }
}

fn fetch_dyndisk_data() -> DynDisk {
    DynDisk {
        disk_used_gb: 10.0,
        disk_io_mbps: 150.4,
    }
}

fn fetch_dynbattery_data() -> DynBattery {
    DynBattery {
        status: BatteryStatus::Discharging,
        battery_temp: 35.2,
    } 
} 

fn fetch_dynnetwork_data() -> DynNetwork {
    DynNetwork {
            download_speed_kbps: 310.0,
            upload_speed_kbps: 350.2,
            ping_ms: 25,
    } 
}


//My name SyFord 17y old thn 2026