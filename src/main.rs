use std::io;
pub mod model;
pub mod ui;
pub mod worker;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use model::{Battery, BatteryStatus, Cpu, Disk, Network, Ram, SystemStatus};
use ratatui::{Terminal, backend::CrosstermBackend};


fn main() -> Result<(), io::Error> {
    
    // Vector untuk menyimpan riwayat 
    //let mut cpu_history: Vec<u64> = Vec::new();
    //let mut download_speed: Vec<u64> = Vec::new();
    //let mut upload_speed: Vec<u64> = Vec::new();


    
    let now_stat = SystemStatus::read_system();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| {
            ui::draw_ui(f, &now_stat);
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
