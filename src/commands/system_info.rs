use comfy_table::{Cell, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};
use sys_info;

pub fn execute() {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS);
    table.set_header(vec![
        Cell::new("Component"),
        Cell::new("Details"),
    ]);

    // Collecting system information
    let os_type = sys_info::os_type().unwrap_or_else(|_| "Unknown".to_string());
    let os_release = sys_info::os_release().unwrap_or_else(|_| "Unknown".to_string());
    let hostname = sys_info::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let cpu_num = sys_info::cpu_num().map_or("Unknown".to_string(), |n| n.to_string());
    let cpu_speed = sys_info::cpu_speed().map_or("Unknown".to_string(), |s| s.to_string());
    let mem_total = sys_info::mem_info().map_or("Unknown".to_string(), |m| format!("{} MB", m.total / 1024));
    let disk_info = sys_info::disk_info().map_or("Unknown".to_string(), |d| format!("{} MB free, {} MB total", d.free / 1024, d.total / 1024));

    // Adding rows to the table
    table.add_row(vec![
        Cell::new("OS Type"),
        Cell::new(os_type),
    ]);
    table.add_row(vec![
        Cell::new("OS Release"),
        Cell::new(os_release),
    ]);
    table.add_row(vec![
        Cell::new("Hostname"),
        Cell::new(hostname),
    ]);
    table.add_row(vec![
        Cell::new("CPU Cores"),
        Cell::new(cpu_num),
    ]);
    table.add_row(vec![
        Cell::new("CPU Speed (MHz)"),
        Cell::new(cpu_speed),
    ]);
    table.add_row(vec![
        Cell::new("Memory Total"),
        Cell::new(mem_total),
    ]);
    table.add_row(vec![
        Cell::new("Disk Info"),
        Cell::new(disk_info),
    ]);

    println!("{}", table);
}