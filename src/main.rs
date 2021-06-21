extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;

fn get_cpu_avg(req_sys: sysinfo::System) -> f32
{

    let mut cpus: Vec<f32> = Vec::new();
    for core in req_sys.get_processors() { cpus.push(core.get_cpu_usage()); }

    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    cpu_avg

}

fn main()
{
    println!("Starting dwm-status... ");
    
    let mut current_sys = sysinfo::System::new_all();
    current_sys.refresh_all();

    let cpu_avg = get_cpu_avg(current_sys);

    println! ("CPU: {:.1}", cpu_avg);
}
