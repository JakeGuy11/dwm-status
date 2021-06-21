extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;

fn main()
{
    println!("Starting dwm-status... ");
    
    let mut current_sys = sysinfo::System::new_all();
    current_sys.refresh_all();

    let mut cpus: Vec<f32> = Vec::new();
    for core in current_sys.get_processors() { cpus.push(core.get_cpu_usage()); }

    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    println! ("CPU: {:.1}", cpu_avg);
}
