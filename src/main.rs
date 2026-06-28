use std::io;
pub mod model;
pub mod ui;
pub mod worker;
use sysinfo::{
    Components, Disks, Networks, System,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use crate::model::{
    Staticdata, Dynamicdata, BatteryStatus, 
    Device, Battery, Cpu, Disk, Ram,
    DynCpu, DynRam, DynDisk, DynBattery, DynNetwork
};

use ratatui::{Terminal, backend::CrosstermBackend};


fn main() -> Result<(), io::Error> {
    let mut sys = System::new_all();
    let mut net = Networks::new_with_refreshed_list();
    let mut disk = Disks::new_with_refreshed_list();



    let stat = Staticdata::read_system(&mut sys, &disk);
    
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {        
        let now_stat = Dynamicdata::read_system(&mut sys, &mut net, &mut disk);
        
        terminal.draw(|f| {
            ui::draw_ui(f, &stat, &now_stat);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
