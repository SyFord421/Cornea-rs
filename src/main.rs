pub mod worker;
pub mod model;
pub mod ui;
use model::{SystemStatus, Cpu, Ram, Disk, Battery, BatteryStatus, Network};


use crossterm:: {
    event::{self, DisableMouseCapture, EnableMouseCapture, event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},    
}


fn main() -> Result {
    
