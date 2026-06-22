use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::model::SystemStatus;

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, status: &SystemStatus) {
    // bagi layar jadi 4 bagian Horizontal
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25), // CPU
            Constraint::Percentage(25), // RAM
            Constraint::Percentage(25), // DISK
            Constraint::Percentage(25), // Battery
        ])
        .split(f.size());

    // widget buat CPU
    let cpu_box = Paragraph::new(format!(
        "cpu usage: {}% load average: {} cpu temp: {}", 
        status.cpu_status.cpu_usage, status.cpu_status.cpu_average, status.cpu_status.cpu_temp
    ))
    .block(Block::default().title(" cpu stat ").borders(Borders::ALL));
    f.render_widget(cpu_box, chunks[0]);
    
    // widget buat RAM
    let ram_box = Paragraph::new(format!("ram usage: {}%", status.ram_status.ram_usage))
        .block(Block::default().title(" ram stat ").borders(Borders::ALL));
    f.render_widget(ram_box, chunks[1]);
    
    // widget buat DISK
    let disk_box = Paragraph::new(format!("disk usage: {}%", status.disk_status.disk_usage))
        .block(Block::default().title(" disk stat ").borders(Borders::ALL));
    f.render_widget(disk_box, chunks[2]);
            
    // widget buat Battery (pake data battery_status juga yank)
    let battery_box = Paragraph::new(format!("battery usage: {}%", status.battery_status.battery_usage))
        .block(Block::default().title(" battery stat ").borders(Borders::ALL));
    f.render_widget(battery_box, chunks[3]);
}
