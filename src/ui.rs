use crate::model::SystemStatus;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn draw_ui(f: &mut Frame, status: &SystemStatus) {
    // bagi layar jadi 3 bagian Horizontal

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40), //CPU
            Constraint::Percentage(40),
            Constraint::Percentage(20), //status layout: battery, network
        ])
        .split(f.size());

    let body_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[0]);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(main_layout[1]);

    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[2]);

    // Untuk body layout bagian CPU
    let cpu = &status.cpu_status;
    let cpu_box = Paragraph::new(format!(
        "cpu usage: {}%\nload average: {}\ncpu temp: {}",
        cpu.cpu_usage, cpu.load_average, cpu.cpu_temp
    ))
    .block(Block::default().title(" CPU ").borders(Borders::ALL));
    f.render_widget(cpu_box, body_layout[0]);

    // Untuk body layout IO storage
    let disk = &status.disk_status;
    let disk_box = Paragraph::new(format!(
        "disk_capacity_gb: {}\ndisk_used_gb: {}\ndisk_i/o: {}MB/s",
        disk.disk_capacity_gb, disk.disk_used_gb, disk.disk_io_mbps,
    ))
    .block(Block::default().title(" DISK ").borders(Borders::ALL));
    f.render_widget(disk_box, body_layout[1]);

    // Untuk Footer bagian Network
    let net = &status.network_status;
    let network_box = Paragraph::new(format!(
        "download_speed: {}KB/s\nPing: {}ms",
        net.download_speed_kbps, net.ping_ms,
    ))
    .block(Block::default().title(" NETWORK ").borders(Borders::ALL));
    f.render_widget(network_box, footer_layout[0]);

    let upload_box = Paragraph::new(format!(
        "Upload Speed: {}MB/s\nPing: {}ms",
        net.upload_speed_kbps, net.ping_ms
    ))
    .block(Block::default().title(" NETWORK ").borders(Borders::ALL));
    f.render_widget(upload_box, footer_layout[2]);

    // Untuk Footer Daftar Task teratas
    let list_items: Vec<ListItem> = status
        .cpu_status
        .top_processes
        .iter()
        .take(5)
        .map(|task| ListItem::new(task.as_str())) // ini loop iterator map
        .collect(); // Kumpulkan Jadi vector baru

    let task_box = List::new(list_items).block(
        Block::default()
            .title(" TOP TODO LIST ")
            .borders(Borders::ALL),
    );
    f.render_widget(task_box, footer_layout[1]);

    //untuk status layout
    let batt = &status.battery_status;
    let battery_box = Paragraph::new(format!(
        "Status: {:?}\nHealth Percentage: {}%\nBattery Temp: {}",
        batt.status, batt.health_percentage, batt.battery_temp
    ))
    .block(Block::default().title(" BATTERY ").borders(Borders::ALL));
    f.render_widget(battery_box, status_layout[0]);

    let time_box = Paragraph::new(format!(
        "Time: {}",
        status.timestamp
        ))
        .block(Block::default().title(" TIME ").borders(Borders::ALL));
    f.render_widget(time_box, status_layout[1]);
}
