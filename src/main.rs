use std::io;
pub mod worker;
pub mod model;
pub mod ui;
use model::{SystemStatus, Cpu, Ram, Disk, Battery, BatteryStatus, Network};
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm:: {
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},    
};



fn main() -> Result<(), io::Error> {
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
            if let Event::Key(key) = event::read()?{
                if key.code == KeyCode::Char('q'){
                    break;
                }
            }
        }   
    } 
    disable_raw_mode()?;
    execute!(terminal.backend_mut(),
             LeaveAlternateScreen,
             DisableMouseCapture
            )?;
    terminal.show_cursor()?;
    Ok(())
}
    
