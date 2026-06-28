use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use crate::model::{
    Staticdata, Dynamicdata, BatteryStatus, 
    Device, Battery, Cpu, Disk, Ram,
    DynCpu, DynRam, DynDisk, DynBattery, DynNetwork
};


pub fn draw_ui(f: &mut Frame, stat: &Staticdata, now_stat: &Dynamicdata) {
    // Persentase disesuaikan supaya pas dan proporsional di layar HP
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15), // Header & Device info
            Constraint::Percentage(25), // CPU
            Constraint::Percentage(25), // DISK
            Constraint::Percentage(25), // Footer (Network & Task 75:25)
            Constraint::Percentage(10), // Status Layout (Battery & Time)
        ])
        .split(f.size());

    // Layout untuk Footer: Membagi Network (75%) dan Task (25%) secara Horizontal
    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(75), // Network luas
            Constraint::Percentage(25), // Task ringkas
        ])
        .split(main_layout[3]);

    // Layout untuk Status paling bawah
    let status_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[4]);

    // --- RENDER WIDGETS ---

    // 0. Header Widget
 
    let header_box = Paragraph::new(format!(
        "SYSTEM: {}\nTIMESTAMP: {}",
        stat.device.name, stat.device.timestamp
    ))
    .block(Block::default().title(" M-MONITOR v1.0 ").borders(Borders::ALL));
    f.render_widget(header_box, main_layout[0]);

    // 1. CPU Widget
    let cpu = &now_stat.cpu_status;
    let cpu_box = Paragraph::new(format!(
        "Usage: {}%\nLoad Average: {}\nTemp: {}°C",
        cpu.cpu_usage, cpu.load_average, cpu.cpu_temp
    ))
    .block(Block::default().title(" CPU ").borders(Borders::ALL));
    f.render_widget(cpu_box, main_layout[1]);

    // 2. DISK Widget
    let now_disk = &now_stat.disk_status;
    let disk_box = Paragraph::new(format!(
        "Total Capacity: {:.2} GB\nUsed: {:.2} GB\nI/O Speed: {} MB/s", stat.disk_status.disk_capacity_gb, now_disk.disk_used_gb, now_disk.disk_io_mbps,
    ))
    .block(Block::default().title(" STORAGE ").borders(Borders::ALL));
    f.render_widget(disk_box, main_layout[2]);

    // 3. NETWORK Widget
    let now_net = &now_stat.network_status;
    let network_box = Paragraph::new(format!(
        "Down: {} KB/s\nUp:   {} MB/s\nPing: {} ms",
        now_net.download_speed_kbps, now_net.upload_speed_kbps, now_net.ping_ms,
    ))
    .block(Block::default().title(" NETWORK ").borders(Borders::ALL));
    f.render_widget(network_box, footer_layout[0]);

    // 4. TASK Widget
    let list_items: Vec<ListItem> = now_stat
        .cpu_status
        .top_processes
        .iter()
        .take(5)
        .map(|task| ListItem::new(format!("• {}", task.as_str())))
        .collect();

    let task_box = List::new(list_items).block(
        Block::default()
            .title(" TASK ")
            .borders(Borders::ALL),
    );
    f.render_widget(task_box, footer_layout[1]);

    // 5. BATTERY Widget
    let batt = &stat.battery_status;
    let now_batt = &now_stat.battery_status;
    let battery_box = Paragraph::new(format!(
        "Status: {:?} | Health: {}% | Temp: {}°C",
        now_batt.status, batt.health_percentage, now_batt.battery_temp
    ))
    .block(Block::default().title(" BATTERY ").borders(Borders::ALL));
    f.render_widget(battery_box, status_layout[0]);

    // 6. TIME/FOOTER tambahan info
    let time_box = Paragraph::new(format!(" System Status: OK"))
        .block(Block::default().title(" STATUS ").borders(Borders::ALL));
    f.render_widget(time_box, status_layout[1]);
}
