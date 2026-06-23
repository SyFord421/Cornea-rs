use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::model::SystemStatus;

pub fn draw_ui(f: &mut Frame, status: &SystemStatus) {
    // bagi layar jadi 4 bagian Horizontal
    let ver_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20), // CPU
            Constraint::Percentage(20), // RAM
            Constraint::Percentage(20), // DISK
        ])
        .split(f.size());

    let hor_chuks = Layout::default() 
        .direction(Direction::Horizontal) 
        .constraints([
            Constraint::Percentage(20), // Battery
            Constraint::Percentage(20), // Network        
        ])
        .split(f.size());

    // widget buat CPU
    let cpu_box = Paragraph::new(format!(
        "cpu usage: {}%\nload average: {}\ncpu temp: {}",
        status.cpu_status.cpu_usage, status.cpu_status.load_average, status.cpu_status.cpu_temp
    ))
    .block(Block::default().title(" cpu stat ").borders(Borders::ALL));
    f.render_widget(cpu_box, ver_chunks[0]);
    
    // widget buat RAM
    let ram_box = Paragraph::new(format!("Ram capacity: {}GB", status.ram_status.ram_capacity_gb))
        .block(Block::default().title(" ram stat ").borders(Borders::ALL));
    f.render_widget(ram_box, ver_chunks[1]);
    
    // widget buat DISK
    let disk_box = Paragraph::new(format!("disk usage: {}%", status.disk_status.disk_capacity_gb))
        .block(Block::default().title(" disk stat ").borders(Borders::ALL));
    f.render_widget(disk_box, hor_chuks[0]);
            
    // widget buat Battery
    let battery_box = Paragraph::new(format!("battery usage: {}%", status.battery_status.health_percentage))
        .block(Block::default().title(" battery stat ").borders(Borders::ALL));
    f.render_widget(battery_box, hor_chuks[1]);

    // widget buat Network
    let battery_box = Paragraph::new(format!("download speed {}KB/s", status.network_status.download_speed_kbps))
        .block(Block::default().title(" battery stat ").borders(Borders::ALL));
    f.render_widget(battery_box, ver_chunks[2C]);


}
