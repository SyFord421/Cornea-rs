
pub mod system_reader;
pub mod data_structure;
use data_structure::{SystemStatus, Cpu, Ram, Disk, Battery, BatteryStatus, Network};


fn main() {
    let status_sekarang = system_reader::read_system(); 
    println!("{:#?}", status_sekarang);
}
