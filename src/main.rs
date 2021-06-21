extern crate cpu_monitor;

use std::thread;
use std::time;
use sysinfo::SystemExt;


fn main() {
    println!("Starting dwm-status... ");
    
    let mut current_sys = sysinfo::System::new();
    current_sys.refresh_all();

    let start = cpu_monitor::CpuInstant::now().unwrap();
    thread::sleep(time::Duration::from_millis(1000));
    let end = cpu_monitor::CpuInstant::now().unwrap();

    let duration = end - start;
    println! ("CPU: {:.1}%\tRAM:12.3%", duration.non_idle() * 100 as f64);
}
